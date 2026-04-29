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

// Hyper-dimensional, self-optimizing memory structure with Information Momentum
#[derive(Clone)]
pub struct TitanMemory {
    pub w: Array1<f32>,
    pub b: Array1<f32>,
    pub w_momentum: Array1<f32>,   // Information Momentum: Identity across time
    pub b_momentum: Array1<f32>,
    pub alpha_field: Array1<f32>,  // Localized, self-optimizing learning rates
    pub base_lr: f32,
    pub momentum_beta: f32,        // Decay for the momentum vector
}

impl TitanMemory {
    pub fn new(size: usize, base_lr: f32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let w = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        let b = Array1::from_shape_fn(size, |_| rng.gen_range(-0.1..0.1));
        let w_momentum = Array1::zeros(size);
        let b_momentum = Array1::zeros(size);
        let alpha_field = Array1::from_elem(size, 1.0); // Start at 1.0 multiplier
        TitanMemory { 
            w, b, w_momentum, b_momentum, alpha_field, base_lr,
            momentum_beta: 0.9 
        }
    }

    pub fn forward(&self, x: &Array1<f32>) -> Array1<f32> {
        let out = x * &self.w + &self.b;
        out.mapv(|v| v.tanh())
    }

    // Returns (thermodynamic_work, modulation field)
    pub fn update_and_modulate(&mut self, x: &Array1<f32>, target: &Array1<f32>) -> (f32, Array1<f32>) {
        let pred = self.forward(x);
        let error = &pred - target;
        
        // 1. Self-Optimizing Memory Structure:
        // Adjust the localized learning rate based on error magnitude.
        for i in 0..self.alpha_field.len() {
            let err_mag = error[i].abs();
            if err_mag > 0.1 {
                self.alpha_field[i] = (self.alpha_field[i] * 1.05).min(5.0);
            } else {
                self.alpha_field[i] = (self.alpha_field[i] * 0.99).max(0.1);
            }
        }
        
        // 2. Information Momentum: Identity as a trajectory
        // The momentum vector accumulates the direction of updates.
        let effective_lr = &self.alpha_field * self.base_lr;
        
        let w_grad = x * &error;
        let b_grad = &error;

        self.w_momentum = &self.w_momentum * self.momentum_beta + (1.0 - self.momentum_beta) * w_grad;
        self.b_momentum = &self.b_momentum * self.momentum_beta + (1.0 - self.momentum_beta) * b_grad;

        // Apply gradient update via momentum
        self.w -= &(&self.w_momentum * &effective_lr);
        self.b -= &(&self.b_momentum * &effective_lr);
        
        // Szilárd's Equivalence: The thermodynamic work of the information update
        let work = error.mapv(|v| v.abs()).mean().unwrap_or(0.0);
        
        (work, pred)
    }
}
