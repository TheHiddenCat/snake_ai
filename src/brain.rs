use rand::Rng;
use rand_distr::Normal;
use ndarray::Array2;

// directions = 8
// data = 3
// inputs = 24 = directions * data = 8 * 3
// hidden = 16 = inputs - directions = 24 - 8
// outputs = 4

#[derive(Debug, Clone)]
pub struct Brain {
    input_weights: Array2<f32>,
    hidden_weights: Array2<f32>,
    input_biases: Array2<f32>,
    hidden_biases: Array2<f32>,
}

impl Brain {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let input_weights = Array2::from_shape_fn((16, 24), |_| rng.gen_range(-1.0..1.0));
        let hidden_weights = Array2::from_shape_fn((4, 16), |_| rng.gen_range(-1.0..1.0));
        let input_biases = Array2::from_shape_fn((16, 1), |_| rng.gen_range(-1.0..1.0));
        let hidden_biases = Array2::from_shape_fn((4, 1), |_| rng.gen_range(-1.0..1.0));

        Brain {
            input_weights,
            hidden_weights,
            input_biases,
            hidden_biases,
        }
    }

    pub fn forward(&self, input: Array2<f32>) -> Array2<f32> {
        let input_layer_output = self.input_weights.dot(&input) + &self.input_biases;
        let hidden_layer_output = self.relu(&input_layer_output);
        let output_layer_output = self.hidden_weights.dot(&hidden_layer_output) + &self.hidden_biases;
        self.relu(&output_layer_output)
    }

    pub fn crossover(&self, other: &Brain) -> Brain {
        let mut rng = rand::thread_rng();

        let input_weights = if rng.gen::<f32>() < 0.5 {
            self.input_weights.clone()
        } else {
            other.input_weights.clone()
        };

        let hidden_weights = if rng.gen::<f32>() < 0.5 {
            self.hidden_weights.clone()
        } else {
            other.hidden_weights.clone()
        };

        let input_biases = if rng.gen::<f32>() < 0.5 {
            self.input_biases.clone()
        } else {
            other.input_biases.clone()
        };

        let hidden_biases = if rng.gen::<f32>() < 0.5 {
            self.hidden_biases.clone()
        } else {
            other.hidden_biases.clone()
        };

        Brain {
            input_weights,
            hidden_weights,
            input_biases,
            hidden_biases,
        }
    }

    pub fn mutate(&mut self, mutation_rate: f32) {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.5).unwrap();

        for weight in self.input_weights.iter_mut() {
            if rng.gen::<f32>() < mutation_rate {
                *weight += rng.sample(normal);
            }
        }

        for weight in self.hidden_weights.iter_mut() {
            if rng.gen::<f32>() < mutation_rate {
                *weight += rng.sample(normal);
            }
        }

        for bias in self.input_biases.iter_mut() {
            if rng.gen::<f32>() < mutation_rate {
                *bias += rng.sample(normal);
            }
        }

        for bias in self.hidden_biases.iter_mut() {
            if rng.gen::<f32>() < mutation_rate {
                *bias += rng.sample(normal);
            }
        }
    }

    #[allow(unused)]
    pub fn sigmoid(&self, x: Array2<f32>) -> Array2<f32> {
        1f32 / (1f32 + (-x).mapv(|x| x.exp()))
    }
    
    pub fn relu(&self, input: &Array2<f32>) -> Array2<f32> {
        input.mapv(|x| if x > 0.0 {
            x
        } else {
            0.0
        })
    }

}