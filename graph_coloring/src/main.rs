mod state;
use state::{Algorithm, Graph};

use std::env;
use std::io;
use std::io::BufRead;

fn usage(tar: &str) {
    match tar {
        "prog" => {
            println!("Usage: cue6_04 ALGORITHM NUM_COLORS
                      Available algorithms are: dfs - Depth first search
                                                fc  - Forward checking
                                                mcv - Most constrained variable")
        },
        "p" => println!("Usage: p edge NODES EDGES"),
        "e" => println!("Usage: e NODE1 NODE2"),
        _ => println!("Unknown usage string"),
    }
}

fn read_stdin() -> Graph {
    let mut graph = Graph::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let directive = match words.get(0) {
            Some(word) => word,
            None => continue,
        };

        match directive.as_ref() {
            "c" => continue,
            "p" => {
                match words.get(1) {
                    Some(&"edge") => {},
                    Some(x) => {
                        println!("Invalid p directive {}", x);
                        continue
                    },
                    None => {
                        usage("p");
                        continue
                    },
                }
                let verts = words.get(2).unwrap().parse::<usize>().unwrap();
                let edges = words.get(3).unwrap().parse::<usize>().unwrap();
                graph.verts(verts).edges(edges);
            },
            "e" => {
                let node1 = words.get(1).unwrap().parse::<usize>().unwrap();
                let node2 = words.get(2).unwrap().parse::<usize>().unwrap();

                graph.connect(node1, node2);
            },
            x => {
                println!("Unknown directive {}", x);
                continue
            },
        };
    }

    graph
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        usage("prog");
        return;
    }

    let algorithm = match argv[1].as_ref() {
        "dfs" => Algorithm::DepthFirst,
        "fc" => Algorithm::ForwardCheck,
        "mcv" => Algorithm::MostConstrained,
        _ => { usage("prog"); return; },
    };
    let num_colors = argv[2].parse::<usize>().unwrap();

    let mut graph = read_stdin();
    graph.colors(num_colors).colorify(algorithm);
}
