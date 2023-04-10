extern crate nalgebra as na;
extern crate rand;

use self::rand::Rng;

pub struct MCPNeuron {
    _learning_rate: f64,
    _size: usize,
    _epochs: i32,
    _input_vector: na::DVector<f64>,
    _weights: na::DVector<f64>,
}

impl MCPNeuron {
    pub fn new(size: usize, epochs: i32, learning_rate: f64) -> MCPNeuron {

        let mut rng = rand::thread_rng();
        return MCPNeuron {
            _size: size,
            _epochs: epochs,
            _learning_rate: learning_rate ,
            _input_vector: na::DVector::identity(size),
            _weights: na::DVector::from_fn(size, |_x, _y| -> f64 { rng.gen::<f64>() })
        }

    }

    pub fn get_weight(&self, index: usize) -> f64 {
        return self._weights[index];
    }
}