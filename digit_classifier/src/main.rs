#![allow(unused_imports)]
mod classifier;

use classifier::{Classifier, Datum};
use std::io::{BufRead, Write};
use std::collections::HashMap;

/// Print to stderr
macro_rules! debug {
    ($($arg:tt)*) => (
        /*
        match writeln!(&mut ::std::io::stdout(), $($arg)*) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
        */
    )
}

const K: usize = 3;

#[derive(Debug, Copy, Clone)]
enum Alg {
    KNN,
    Linear,
}

fn read_args() -> Option<Alg> {
    let args: Vec<String> = std::env::args().collect();
    args.get(1).and_then(|arg| {
        match arg.as_ref() {
            "knn" => Some(Alg::KNN),
            "linear" => Some(Alg::Linear),
            _ => None
        }
    })
}

fn read_description() -> (Vec<Datum>, usize, usize, usize) {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines().map(|line| line.unwrap());

    let (attributes, values, classes) = read_header(&line_iter.next().unwrap());
    match line_iter.next().unwrap().as_ref() {
        "-- training --" => {},
        _ => panic!("Expected training indicator line"),
    }

    let mut data = Vec::new();
    loop {
        match line_iter.next().unwrap().as_ref() {
            "-- test --" => break,
            line => {
                debug!("READ LINE: {}", line);
                let (id, datum) = read_train_datum(line, attributes, values);
                data.push(Datum::new(id, datum));
            },
        }
    }
    (data, classes, attributes, values)
}

fn read_header(line: &str) -> (usize, usize, usize) {
    let words: Vec<&str> = line.split_whitespace().collect();
    let attr = words.get(0).unwrap().parse::<usize>().unwrap();
    let val = words.get(2).unwrap().parse::<usize>().unwrap();
    let classes = words.get(4).unwrap().parse::<usize>().unwrap();

    (attr, val, classes)
}

fn read_train_datum(data: &str, attr: usize, values: usize)
    -> (usize, Vec<f64>) {
    let mut words = data.split_whitespace()
                        .map(|word| word.parse::<usize>().unwrap());
    let id = words.next().unwrap();
    let vector: Vec<f64> = words.map(|x| normalize(x, values)).collect();
    assert!(vector.len() == attr);
    (id, vector)
}

fn read_test_datum(data: &str, attr: usize, values: usize) -> Vec<f64> {
    let data: Vec<f64> = data.split_whitespace()
                               .map(|word| word.parse::<usize>().unwrap())
                               .map(|x| normalize(x, values))
                               .collect();
    assert!(data.len() == attr);
    data
}

/// Normalize num from 0 to range
fn normalize(num: usize, range: usize) -> f64 {
    (num as f64) / ((range - 1) as f64)
}

fn knn(train: &Vec<Datum>, k: usize, test: &Datum) -> (usize, f64) {
    // Create an array of distance to class pairs
    let mut euclids: Vec<(f64, usize)> =
        train.iter()
             .map(|datum| (euclid_distance(datum.get_vec(), test.get_vec()),
                           datum.id()))
             .collect();

    // A selection sort performed k times.
    for i in 0..k {
        let mut min = euclids.get(i).unwrap().0;
        let mut min_index = i;
        // NOTE: use iter.enumerate() here
        for j in (i + 1)..euclids.len() {
            let &(dist, _) = unsafe { euclids.get_unchecked(j) };
            if dist < min {
                min = dist;
                min_index = j;
            }
        }
        euclids.swap(i, min_index);
    }

    // Needs a datastructure to count occurances. HashMap chosen arbitrarily
    // The hash data maps classes to distance, occurance pairs
    let mut scores: HashMap<usize, (f64, usize)> = HashMap::new();
    let mut total_dist = 0_f64;
    for &(dist, class) in euclids.iter().take(k) {
        if scores.contains_key(&class) {
            let node = scores.get_mut(&class).unwrap();
            *node = (node.0 + dist, node.1 + 1)
        } else {
            scores.insert(class, (dist, 1));
        }
        total_dist += dist;
    }

    let (class, (dist, _)) = find_max(&scores);
    (class, (1_f64 - (dist / total_dist)))
}

fn euclid_distance(p1: &Vec<f64>, p2: &Vec<f64>) -> f64 {
    let mut sum = 0_f64;
    for (a, b) in p1.iter().zip(p2.iter()) {
        // this will totally overflow
        sum += (a - b).powi(2);
    }
    sum.sqrt()
}

/// Finds max usize value in HashMap with lowest f64 values breaking ties.
fn find_max(scores: &HashMap<usize, (f64, usize)>) -> (usize, (f64, usize)) {
    let mut max_class = 12345;
    let mut max_dist = 0_f64;
    let mut max_occurance = 0;
    for (class, &(dist, occ)) in scores.iter() {
        if occ > max_occurance {
            max_class = *class;
            max_dist = dist;
            max_occurance = occ;
        } else if occ == max_occurance {
            if dist < max_dist {
                max_class = *class;
                max_dist = dist;
                max_occurance = occ;
            }
        }
    }

    debug!("{} with dist {}, {} times", max_class, max_dist, max_occurance);

    (max_class, (max_dist, max_occurance))
}

/*
fn normalize_training_set(train: &mut Vec<Datum>) {
    for i in 0..train.get(0).unwrap().get_vec().len() {
        let mean = train.iter().fold(0.0, |acc, ref datum| acc + datum.get_vec().get(i).unwrap()) / (train.len() as f64);
        let variance = (train.iter().fold(0.0, |acc, ref datum| {
            acc + (datum.get_vec().get(i).unwrap() - mean).powi(2)
        }) / (train.len() as f64)).sqrt();
        println!("variance {}", variance);
        for datum in train.iter_mut() {
            let item = datum.get_vec_mut().get_mut(i).unwrap();
            *item = *item / variance;
        }
    }
}
*/

fn main() {
    let alg = read_args();
    if alg.is_none() {
        println!("Usage: cue6_09 [ALG]");
        return;
    }



    let (mut training_data, class, attr, val) = read_description();
    match alg.unwrap() {
        Alg::KNN => {
            // Read test points until EOF
            loop {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(n) => if n == 0 {
                        break;
                    } else {
                        let test = Datum::new(0, read_test_datum(&input, attr, val));
                        let (class, confidence) = knn(&mut training_data, K, &test);
                        println!("{} {}", class, confidence);
                    },
                    Err(e) => panic!("Error stdin {}", e),
                }
            }
        },
        Alg::Linear => {
            let classifiers: Vec<Classifier> =
                (0..class).map(|n| {
                    let mut classifier = Classifier::new_from(n, &training_data);
                    classifier.learn();
                    classifier
                }).collect();
            // Read test points until EOF
            loop {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(n) => if n == 0 {
                        break;
                    } else {
                        let test = Datum::new(0, read_test_datum(&input, attr, val));
                        let outputs = classifiers.iter().map(|classifier| classifier.test(&test));

                        /*
                        let outputs_vec: Vec<f64> = outputs.collect();
                        println!("{:?}", outputs_vec);
                        */
                        let mut max = std::f64::MIN;
                        let mut max_class = 0;
                        for (classifier, output) in classifiers.iter().zip(outputs) {
                            if output > max {
                                max = output;
                                max_class = classifier.id();
                            }
                        }
                        println!("{} {}", max_class, (1. + max) / 2. );
                    },
                    Err(e) => panic!("Error stdin {}", e),
                }
            }
        }
    }
}
