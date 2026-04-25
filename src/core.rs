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

// Hyper-dimensional, self-optimizing memory structure
#[derive(Clone)]
pub struct TitanMemory {
    pub w: Array1<f32>,
    pub b: Array1<f32>,
    pub alpha_field: Array1<f32>, // Localized, self-optimizing learning rates
    pub base_lr: f32,
}

impl TitanMemory {
    pub fn new(size: usize, base_lr: f32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let w = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        let b = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        let alpha_field = Array1::from_elem(size, 1.0); // Start at 1.0 multiplier
        TitanMemory { w, b, alpha_field, base_lr }
    }

    pub fn forward(&self, x: &Array1<f32>) -> Array1<f32> {
        let out = x * &self.w + &self.b;
        out.mapv(|v| v.tanh())
    }

    // Returns (scalar loss, modulation field)
    pub fn update_and_modulate(&mut self, x: &Array1<f32>, target: &Array1<f32>) -> (f32, Array1<f32>) {
        let pred = self.forward(x);
        let error = &pred - target;
        
        // 1. Self-Optimizing Memory Structure:
        // Adjust the localized learning rate based on error magnitude.
        // High error = increase learning rate. Low error = decay learning rate (consolidation).
        for i in 0..self.alpha_field.len() {
            let err_mag = error[i].abs();
            if err_mag > 0.1 {
                self.alpha_field[i] = (self.alpha_field[i] * 1.05).min(5.0); // Cap multiplier
            } else {
                self.alpha_field[i] = (self.alpha_field[i] * 0.99).max(0.1); // Floor multiplier
            }
        }
        
        // Apply gradient update using the dynamic, localized alpha field
        let effective_lr = &self.alpha_field * self.base_lr;
        self.w -= &(x * &error * &effective_lr);
        self.b -= &(&error * &effective_lr);
        
        let loss = error.mapv(|v| v.abs()).mean().unwrap_or(0.0);
        
        // The output of the memory network becomes the modulation field for the next step
        (loss, pred)
    }
}
