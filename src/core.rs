use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Cell {
    pub u_re: f32, pub u_im: f32,
    pub d_re: f32, pub d_im: f32,
    pub l_re: f32, pub l_im: f32,
    pub r_re: f32, pub r_im: f32,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            u_re: 0.0, u_im: 0.0,
            d_re: 0.0, d_im: 0.0,
            l_re: 0.0, l_im: 0.0,
            r_re: 0.0, r_im: 0.0,
        }
    }
    pub fn prob(&self) -> f32 {
        self.u_re.powi(2) + self.u_im.powi(2) + self.d_re.powi(2) + self.d_im.powi(2) +
        self.l_re.powi(2) + self.l_im.powi(2) + self.r_re.powi(2) + self.r_im.powi(2)
    }
}

pub struct TitanMemory {
    pub w: Array1<f32>,
    pub b: Array1<f32>,
    pub lr: f32,
}

impl TitanMemory {
    pub fn new(size: usize, lr: f32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let w = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        let b = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        TitanMemory { w, b, lr }
    }

    pub fn forward(&self, x: &Array1<f32>) -> Array1<f32> {
        let out = x * &self.w + &self.b;
        out.mapv(|v| v.tanh())
    }

    pub fn update(&mut self, x: &Array1<f32>, target: &Array1<f32>) -> f32 {
        let pred = self.forward(x);
        let error = &pred - target;
        
        self.w -= &(x * &error * self.lr);
        self.b -= &(&error * self.lr);
        
        error.mapv(|v| v.abs()).mean().unwrap_or(0.0)
    }
}
