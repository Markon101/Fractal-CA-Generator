use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Cell {
    pub u_re: f32, pub u_im: f32,
    pub d_re: f32, pub d_im: f32,
    pub l_re: f32, pub l_im: f32,
    pub r_re: f32, pub r_im: f32,
    pub source_idx: i32, // Provenance: Index of the original prompt byte
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            u_re: 0.0, u_im: 0.0,
            d_re: 0.0, d_im: 0.0,
            l_re: 0.0, l_im: 0.0,
            r_re: 0.0, r_im: 0.0,
            source_idx: -1,
        }
    }
    pub fn prob(&self) -> f32 {
        self.u_re.powi(2) + self.u_im.powi(2) + self.d_re.powi(2) + self.d_im.powi(2) +
        self.l_re.powi(2) + self.l_im.powi(2) + self.r_re.powi(2) + self.r_im.powi(2)
    }
}

use safetensors::tensor::{Dtype, TensorView, SafeTensors};
use safetensors::serialize_to_file;
use std::collections::HashMap;

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

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let w_bytes: &[u8] = bytemuck::cast_slice(self.w.as_slice().unwrap());
        let b_bytes: &[u8] = bytemuck::cast_slice(self.b.as_slice().unwrap());
        let w_mom_bytes: &[u8] = bytemuck::cast_slice(self.w_momentum.as_slice().unwrap());
        let b_mom_bytes: &[u8] = bytemuck::cast_slice(self.b_momentum.as_slice().unwrap());
        let alpha_bytes: &[u8] = bytemuck::cast_slice(self.alpha_field.as_slice().unwrap());

        let mut data = HashMap::new();
        data.insert("w".to_string(), TensorView::new(Dtype::F32, vec![self.w.len()], w_bytes).unwrap());
        data.insert("b".to_string(), TensorView::new(Dtype::F32, vec![self.b.len()], b_bytes).unwrap());
        data.insert("w_momentum".to_string(), TensorView::new(Dtype::F32, vec![self.w_momentum.len()], w_mom_bytes).unwrap());
        data.insert("b_momentum".to_string(), TensorView::new(Dtype::F32, vec![self.b_momentum.len()], b_mom_bytes).unwrap());
        data.insert("alpha_field".to_string(), TensorView::new(Dtype::F32, vec![self.alpha_field.len()], alpha_bytes).unwrap());

        serialize_to_file(&data, None::<HashMap<String, String>>, std::path::Path::new(path)).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    pub fn load(path: &str, base_lr: f32) -> std::io::Result<Self> {
        let buffer = std::fs::read(path)?;
        let tensors = SafeTensors::deserialize(&buffer).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let get_arr = |name: &str| -> std::io::Result<Array1<f32>> {
            let tensor = tensors.tensor(name).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            let slice: &[f32] = bytemuck::cast_slice(tensor.data());
            Ok(Array1::from_vec(slice.to_vec()))
        };

        let w = get_arr("w")?;
        let b = get_arr("b")?;
        let w_momentum = get_arr("w_momentum")?;
        let b_momentum = get_arr("b_momentum")?;
        let alpha_field = get_arr("alpha_field")?;

        Ok(TitanMemory {
            w,
            b,
            w_momentum,
            b_momentum,
            alpha_field,
            base_lr,
            momentum_beta: 0.9,
        })
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
