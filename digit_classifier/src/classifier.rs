pub struct Classifier<'data> {
    id: usize,
    train: &'data Vec<Datum>,
    weights: Vec<f64>,
    learning_rate: f64,
}

impl<'data> Classifier<'data> {
    pub fn new_from(id: usize, data: &Vec<Datum>) -> Classifier {
        // extra weight for an imaginary train value
        let init_vec = ::std::iter::repeat(0.0)
                                    .take(data.get(0).unwrap().get_vec().len() + 1)
                                    .collect();
        Classifier {
            id: id.clone(),
            train: data,
            weights: init_vec,
            learning_rate: 0.01
        }
    }
    pub fn learn(&mut self) {
        // Arbitrary value
        for datum in self.train.iter() {
            let prediction = self.test(datum);
            let goal = if datum.id() == self.id { 1_f64 } else { -1_f64 };
            let error = prediction - goal;
            //println!("Got {} data, predict {}, err {}", datum.id(), prediction, error);
            for (i, weight) in self.weights.iter_mut().enumerate() {
                let data_point = if i == 0 { 1.0 } else { *(datum.get_vec().get(i - 1).unwrap()) };
                *weight -= self.learning_rate * error * data_point;
            }
            //println!("{} {:?}", self.id, self.weights);
        }
    }

    pub fn test(&self, datum: &Datum) -> f64 {
        let imaginary = vec![1.0];
        imaginary.iter().chain(datum.get_vec())
             .zip(self.weights.iter())
             .fold(0_f64, |acc, (&pixel, &weight)| acc + pixel * weight)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct Datum {
    id: usize,
    vector: Vec<f64>
}

impl Datum {
    pub fn new(id: usize, vec: Vec<f64>) -> Datum {
        Datum {
            id: id,
            vector: vec
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get_vec(&self) -> &Vec<f64> {
        &self.vector
    }
}
