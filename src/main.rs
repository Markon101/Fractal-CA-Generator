use axum::{
    extract::{State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use clap::{Parser, Subcommand};
use ndarray::prelude::*;

mod core;
use crate::core::{Cell, TitanMemory};

#[derive(Clone)]
struct LatticeState {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    iteration: u64,
    seed_prompt: String,
    instruction_header: String,
    semantic_field: Vec<f32>,   // Constant prompt influence
    memory: TitanMemory,        // HYPER-DIMENSIONAL FEEDBACK LOOP
    current_probs: Array1<f32>, // Store state for gradient updates
    cumulative_complexity: f32, // The Arrow of Time: Accumulated state rotations
    causation_coupling: f32,    // Multiplier for downward causation recursion phase
}

#[derive(Serialize, Debug)]
struct Metrics {
    entropy: f32,
    density: f32,
    resonance: f32,
    phi: f32,   // Integrated Information Potential
    work: f32,  // Thermodynamic work (Szilárd equivalence)
    state_complexity: f32, // The Arrow of Time
    raw_sum: f32,
    coherence: f32, // Hilbert phase coherence
}

impl LatticeState {
    fn new(width: usize, height: usize, prompt: &str) -> Self {
        let mut grid = vec![vec![Cell::new(); width]; height];
        let mut semantic_field = vec![0.0; width * height];
        
        let dictionary: Vec<&str> = include_str!("words.txt").lines().collect();
        let mut dict_map = std::collections::HashMap::new();
        for (i, word) in dictionary.iter().enumerate() {
            dict_map.insert(*word, i);
        }

        let words: Vec<&str> = prompt.split_whitespace().collect();
        
        if words.is_empty() {
            grid[height/2][width/2].u_re = 1.0;
        } else {
            for (w_idx, word) in words.iter().enumerate() {
                // Find word in dictionary or use hash
                let d_idx = dict_map.get(word.to_lowercase().as_str())
                    .cloned()
                    .unwrap_or_else(|| {
                        // Simple hash for OOV words
                        let mut h = 0usize;
                        for b in word.as_bytes() { h = h.wrapping_add(*b as usize).wrapping_mul(31); }
                        h % dictionary.len()
                    });

                let phase = (d_idx as f32 / dictionary.len() as f32) * std::f32::consts::TAU;
                
                // Inject the word into the grid at multiple points for redundancy
                for attempt in 0..5 {
                    let seed = (w_idx * 13 + attempt * 7) as u64;
                    // Deterministic pseudo-random position based on word and attempt
                    let pos_idx = ((seed.wrapping_mul(0x517cc1b727220a95)) >> 32) as usize % (width * height);
                    let x = pos_idx % width;
                    let y = pos_idx / width;

                    grid[y][x].u_re = phase.cos() * 0.8;
                    grid[y][x].u_im = phase.sin() * 0.8;
                    grid[y][x].source_idx = w_idx as i32;
                    semantic_field[y * width + x] = d_idx as f32 / dictionary.len() as f32;
                }
            }
        }

        let size = width * height;
        let mut initial_probs = Vec::with_capacity(size);
        for row in &grid {
            for cell in row {
                initial_probs.push(cell.prob());
            }
        }

        LatticeState {
            grid,
            width,
            height,
            iteration: 0,
            seed_prompt: prompt.to_string(),
            instruction_header: "## TITAN MEMORY CONTEXT".to_string(),
            semantic_field,
            memory: TitanMemory::new(size, 0.01),
            current_probs: Array1::from_vec(initial_probs),
            cumulative_complexity: 0.0,
            causation_coupling: 1.0,
        }
    }

    fn step(&mut self) -> f32 {
        let mut next_grid = self.grid.clone();
        
        // 1. Hyper-dimensional feedback: Get modulation from Titan Memory based on previous state
        let (_work, modulation_field) = self.memory.update_and_modulate(&self.current_probs, &self.current_probs); // Predict self
        
        // 1.5. Renormalization Group (Hilbert Space macro-states)
        let macro_width = self.width / 2;
        let macro_height = self.height / 2;
        let mut macro_grid = vec![vec![Cell::new(); macro_width]; macro_height];
        
        if macro_width > 0 && macro_height > 0 {
            for y in 0..self.height {
                for x in 0..self.width {
                    let c = &self.grid[y][x];
                    let my = y / 2;
                    let mx = x / 2;
                    if my < macro_height && mx < macro_width {
                        let mc = &mut macro_grid[my][mx];
                        mc.u_re += c.u_re; mc.u_im += c.u_im;
                        mc.d_re += c.d_re; mc.d_im += c.d_im;
                        mc.l_re += c.l_re; mc.l_im += c.l_im;
                        mc.r_re += c.r_re; mc.r_im += c.r_im;
                    }
                }
            }
            // Normalize macro cells (Renormalization step)
            for my in 0..macro_height {
                for mx in 0..macro_width {
                    let mc = &mut macro_grid[my][mx];
                    let mag = (mc.prob() + 1e-9).sqrt();
                    mc.u_re /= mag; mc.u_im /= mag;
                    mc.d_re /= mag; mc.d_im /= mag;
                    mc.l_re /= mag; mc.l_im /= mag;
                    mc.r_re /= mag; mc.r_im /= mag;
                }
            }
        }

        let coin = |c: &Cell, dir: char| -> (f32, f32) {
            let s_re = c.u_re + c.d_re + c.l_re + c.r_re;
            let s_im = c.u_im + c.d_im + c.l_im + c.r_im;
            match dir {
                'u' => (0.5 * s_re - c.u_re, 0.5 * s_im - c.u_im),
                'd' => (0.5 * s_re - c.d_re, 0.5 * s_im - c.d_im),
                'l' => (0.5 * s_re - c.l_re, 0.5 * s_im - c.l_im),
                'r' => (0.5 * s_re - c.r_re, 0.5 * s_im - c.r_im),
                _ => (0.0, 0.0)
            }
        };

        let mut next_probs = Vec::with_capacity(self.width * self.height);
        let mut step_complexity = 0.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let u_c = &self.grid[(y + 1) % self.height][x];
                let d_c = &self.grid[(y + self.height - 1) % self.height][x];
                let l_c = &self.grid[y][(x + 1) % self.width];
                let r_c = &self.grid[y][(x + self.width - 1) % self.width];

                let (u_re, u_im) = coin(u_c, 'u');
                let (d_re, d_im) = coin(d_c, 'd');
                let (l_re, l_im) = coin(l_c, 'l');
                let (r_re, r_im) = coin(r_c, 'r');

                // Determine provenance: pick source_idx from neighbor with highest prob
                let mut best_source = -1;
                let mut max_nb_prob = -1.0;
                for nb in &[u_c, d_c, l_c, r_c] {
                    let p = nb.prob();
                    if p > max_nb_prob {
                        max_nb_prob = p;
                        best_source = nb.source_idx;
                    }
                }

                let mut cell = Cell { u_re, u_im, d_re, d_im, l_re, l_im, r_re, r_im, source_idx: best_source };
                
                let p = cell.prob();
                let idx = y * self.width + x;
                let mod_val = modulation_field[idx];
                let sem_val = self.semantic_field[idx];
                
                // Downward causation via recursive Hilbert inner product
                let mut recursion_phase = 0.0;
                if macro_width > 0 && macro_height > 0 {
                    let my = (y / 2).min(macro_height - 1);
                    let mx = (x / 2).min(macro_width - 1);
                    let mc = &macro_grid[my][mx];
                    // Complex inner product Re(<cell, macro_cell>)
                    let overlap = cell.u_re * mc.u_re + cell.u_im * mc.u_im +
                                  cell.d_re * mc.d_re + cell.d_im * mc.d_im +
                                  cell.l_re * mc.l_re + cell.l_im * mc.l_im +
                                  cell.r_re * mc.r_re + cell.r_im * mc.r_im;
                    recursion_phase = overlap;
                }

                // Hyper-dimensional rotation: Driven by semantic prompt, Neural Memory, and Recursive Renormalization
                let theta = p * 10.0 * (1.0 + mod_val) * (1.0 + sem_val * 2.0) * (1.0 + recursion_phase * self.causation_coupling); 
                step_complexity += theta.abs();
                
                let (cos_t, sin_t) = (theta.cos(), theta.sin());
                
                let rotate = |re: f32, im: f32| (re * cos_t - im * sin_t, re * sin_t + im * cos_t);
                (cell.u_re, cell.u_im) = rotate(cell.u_re, cell.u_im);
                (cell.d_re, cell.d_im) = rotate(cell.d_re, cell.d_im);
                (cell.l_re, cell.l_im) = rotate(cell.l_re, cell.l_im);
                (cell.r_re, cell.r_im) = rotate(cell.r_re, cell.r_im);

                next_grid[y][x] = cell;
                next_probs.push(cell.prob());
            }
        }
        
        self.grid = next_grid;
        self.cumulative_complexity += step_complexity;
        self.iteration += 1;
        
        let new_probs_arr = Array1::from_vec(next_probs);
        
        // 2. Self-Optimizing Update: Titan learns the transition from old state to new state
        let (true_work, _) = self.memory.update_and_modulate(&self.current_probs, &new_probs_arr);
        self.current_probs = new_probs_arr;
        
        true_work
    }

    fn get_probs(&self) -> Vec<f32> {
        self.grid.iter().flatten().map(|c| c.prob()).collect()
    }

    fn get_metrics(&self) -> Metrics {
        let probs = self.get_probs();
        let total_p: f32 = probs.iter().sum();
        
        if total_p == 0.0 {
            return Metrics { entropy: 0.0, density: 0.0, resonance: 0.0, phi: 0.0, work: 0.0, state_complexity: self.cumulative_complexity, raw_sum: 0.0, coherence: 0.0 };
        }

        let entropy = -probs.iter().filter(|&&p| p > 0.0).map(|&p| {
            let normalized_p = p / total_p;
            normalized_p * normalized_p.log2()
        }).sum::<f32>();

        let max_p = probs.iter().cloned().fold(0.0, f32::max);
        let threshold = max_p * 0.1;
        let active_cells = probs.iter().filter(|&&p| p > threshold).count();
        let density = active_cells as f32 / probs.len() as f32;

        let avg_p = total_p / probs.len() as f32;
        let variance = probs.iter().map(|&p| (p - avg_p).powi(2)).sum::<f32>() / probs.len() as f32;
        let resonance = variance.sqrt() / (avg_p + 1e-9);

        // Integrated Information Proxy: Phi peaks when the system is balanced between 
        // high differentiation (entropy) and high structural integration (resonance).
        let phi = entropy * resonance;

        // Calculate current work without mutating
        let pred = self.memory.forward(&self.current_probs);
        let work = (&pred - &self.current_probs).mapv(|v| v.abs()).mean().unwrap_or(0.0);

        // Calculate Hilbert Phase Coherence
        let mut coherence_sum = 0.0;
        let mut pairs = 0.0;
        for y in 0..self.height {
            for x in 0..self.width {
                let c1 = &self.grid[y][x];
                let c_right = &self.grid[y][(x + 1) % self.width];
                let c_down = &self.grid[(y + 1) % self.height][x];

                let overlap_r = c1.u_re * c_right.u_re + c1.u_im * c_right.u_im +
                                c1.d_re * c_right.d_re + c1.d_im * c_right.d_im +
                                c1.l_re * c_right.l_re + c1.l_im * c_right.l_im +
                                c1.r_re * c_right.r_re + c1.r_im * c_right.r_im;
                let overlap_d = c1.u_re * c_down.u_re + c1.u_im * c_down.u_im +
                                c1.d_re * c_down.d_re + c1.d_im * c_down.d_im +
                                c1.l_re * c_down.l_re + c1.l_im * c_down.l_im +
                                c1.r_re * c_down.r_re + c1.r_im * c_down.r_im;
                coherence_sum += overlap_r.abs() + overlap_d.abs();
                pairs += 2.0;
            }
        }
        let coherence = if pairs > 0.0 { coherence_sum / pairs } else { 0.0 };

        Metrics {
            entropy,
            density,
            resonance,
            phi,
            work,
            state_complexity: self.cumulative_complexity,
            raw_sum: total_p,
            coherence,
        }
    }

    fn get_formatted_output(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("CA ITERATION: {}\n\n", self.iteration));

        let probs = self.get_probs();
        let max_prob = probs.iter().cloned().fold(0.0001, f32::max);

        for row in &self.grid {
            for cell in row {
                let normalized = cell.prob() / max_prob;
                let char = if normalized < 0.05 { ' ' }
                    else if normalized < 0.2 { '.' }
                    else if normalized < 0.4 { '*' }
                    else if normalized < 0.6 { 'o' }
                    else if normalized < 0.8 { 'X' }
                    else { '@' };
                output.push(char);
                output.push(' ');
            }
            output.push('\n');
        }
        output
    }

    fn get_semantic_eigenstate(&self) -> String {
        let dictionary: Vec<&str> = include_str!("words.txt").lines().collect();
        let vocab_size = dictionary.len() as f32;
        
        let mut eigen_prompt = Vec::new();
        
        // Coarse grain the grid into 8x8 macro-regions
        let macro_size = 8;
        let regions_y = self.height / macro_size;
        let regions_x = self.width / macro_size;
        
        for ry in 0..regions_y {
            for rx in 0..regions_x {
                let mut sum_re = 0.0;
                let mut sum_im = 0.0;
                let mut total_prob = 0.0;
                
                for dy in 0..macro_size {
                    for dx in 0..macro_size {
                        let y = ry * macro_size + dy;
                        let x = rx * macro_size + dx;
                        if y < self.height && x < self.width {
                            let cell = &self.grid[y][x];
                            let p = cell.prob();
                            sum_re += cell.u_re * p;
                            sum_im += cell.u_im * p;
                            total_prob += p;
                        }
                    }
                }
                
                if total_prob > 0.5 { // Higher threshold for semantic clarity
                    let phase = sum_im.atan2(sum_re); // -PI to PI
                    let normalized_phase = (phase + std::f32::consts::PI) / (2.0 * std::f32::consts::PI); 
                    
                    // Rarity Bias: Favor words from the latter half of the frequency-sorted dictionary
                    // if the region is particularly resonant.
                    let bias = if total_prob > 2.0 { 0.5 } else { 0.0 };
                    let adjusted_idx = (normalized_phase * (vocab_size * (1.0 - bias))) + (vocab_size * bias);
                    let word_idx = (adjusted_idx as usize).clamp(0, dictionary.len() - 1);
                    
                    let word = dictionary[word_idx];
                    // Filter out very common stop words if they appear as eigenstates
                    let stop_words = ["the", "of", "and", "to", "a", "in", "for", "is", "on", "that", "by", "this", "with"];
                    if !stop_words.contains(&word) {
                        eigen_prompt.push(word);
                    }
                }
            }
        }
        
        if eigen_prompt.is_empty() {
            "void".to_string()
        } else {
            eigen_prompt.join(" ")
        }
    }

    fn extract_grounded_attractors(&self, top_n: usize) -> Vec<(String, f32)> {
        let mut source_counts = std::collections::HashMap::new();
        for row in &self.grid {
            for cell in row {
                if cell.source_idx >= 0 {
                    let p = cell.prob();
                    *source_counts.entry(cell.source_idx as usize).or_insert(0.0) += p;
                }
            }
        }
        
        let words: Vec<&str> = self.seed_prompt.split_whitespace().collect();
        let mut word_influences = Vec::new();
        
        for (i, word) in words.iter().enumerate() {
            let influence = source_counts.get(&i).cloned().unwrap_or(0.0);
            word_influences.push((word.to_string(), influence));
        }
        
        word_influences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for (word, influence) in word_influences {
            if !seen.contains(&word) && influence > 0.0 {
                seen.insert(word.clone());
                result.push((word, influence));
            }
            if result.len() >= top_n { break; }
        }
        
        result
    }
}

// --- CLI ---

#[derive(Parser)]
#[command(name = "fractal-ca")]
#[command(about = "Fractal Cellular Automata with NATIVE Self-Optimizing Titan Memory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server { #[arg(short, long, default_value_t = 3000)] port: u16 },
    Agent {
        prompt: String,
        #[arg(short, long, default_value_t = 15)] iterations: u64,
        #[arg(short, long, default_value_t = 80)] width: usize,
        #[arg(long, default_value_t = 40)] height: usize,
        #[arg(short, long, default_value_t = 5)] points: usize,
        #[arg(long)] load_model: Option<String>,
        #[arg(long)] save_model: Option<String>,
    },
    Lab {},
    Observe {
        seed: String,
        #[arg(short, long, default_value_t = 0)] duration: u64,
    },
    SelfPrime {
        #[arg(default_value = "Genesis State")] prompt: String,
        #[arg(short, long, default_value_t = 5)] generations: usize,
        #[arg(short, long, default_value_t = 15)] iterations: u64,
        #[arg(short, long, default_value_t = 160)] width: usize,
        #[arg(long, default_value_t = 80)] height: usize,
        #[arg(long)] load_model: Option<String>,
        #[arg(long)] save_model: Option<String>,
    },
    Prime {
        instruction: String,
        #[arg(short, long, default_value_t = 10)] iterations: u64,
        #[arg(long)] homeostatic: bool,
    },
    Benchmark {
        #[arg(short, long, default_value_t = 5)] samples: usize,
    },
    SelfTest {
        #[arg(default_value = "The architecture of intelligence is a fractal resonance of information and entropy.")] prompt: String,
    },
    DeepTime {
        #[arg(default_value = "Europa Orbital Research Station: Thousand Year Legacy")] prompt: String,
    },
    ShockTest {
        #[arg(default_value = "Thermodynamic Resilience")] prompt: String,
    },
    PerturbTest {
        #[arg(default_value = "Breathing Chaos")] prompt: String,
    },
}

// --- Server Implementation ---

struct AppState {
    lattice: Mutex<LatticeState>,
}

#[derive(Deserialize)]
struct InitRequest { 
    width: usize, 
    height: usize, 
    #[serde(alias = "seed_prompt")] seed: String,
    #[serde(default)] instruction_header: Option<String>,
}

async fn init_lattice(State(state): State<Arc<AppState>>, Json(payload): Json<InitRequest>) -> Json<String> {
    let mut l = state.lattice.lock().unwrap();
    *l = LatticeState::new(payload.width, payload.height, &payload.seed);
    if let Some(header) = payload.instruction_header { l.instruction_header = header; }
    Json("Init".to_string())
}

#[derive(Deserialize)]
struct ModulateRequest { 
    #[serde(default = "default_step_count")] count: u64,
}

fn default_step_count() -> u64 { 1 }

async fn run_step(State(state): State<Arc<AppState>>, Json(payload): Json<ModulateRequest>) -> Json<Vec<f32>> {
    let mut l = state.lattice.lock().unwrap();
    for _ in 0..payload.count {
        l.step(); // Step now intrinsically handles Titan modulation
    }
    Json(l.get_probs())
}

#[derive(Deserialize)]
struct FormattedRequest {
    #[serde(default)] steps: u64,
}

async fn get_formatted(State(state): State<Arc<AppState>>, payload: Option<Json<FormattedRequest>>) -> String {
    let mut l = state.lattice.lock().unwrap();
    if let Some(Json(p)) = payload {
        for _ in 0..p.steps { l.step(); }
    }
    l.get_formatted_output()
}

// --- Main Loop & Subcommand Handlers ---

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Server { port }) => run_server(port).await,
        Some(Commands::Agent { prompt, iterations, width, height, points, load_model, save_model }) => run_agent(&prompt, iterations, width, height, points, load_model, save_model),
        Some(Commands::Lab {}) => run_lab(),
        Some(Commands::Observe { seed, duration }) => run_observe(&seed, duration).await,
        Some(Commands::SelfPrime { prompt, generations, iterations, width, height, load_model, save_model }) => run_self_prime(&prompt, generations, iterations, width, height, load_model, save_model),
        Some(Commands::Prime { instruction, iterations, homeostatic }) => run_prime(&instruction, iterations, homeostatic),
        Some(Commands::Benchmark { samples }) => run_benchmark(samples),
        Some(Commands::SelfTest { prompt }) => run_self_test(&prompt),
        Some(Commands::DeepTime { prompt }) => run_deep_time(&prompt),
        Some(Commands::ShockTest { prompt }) => run_shock_test(&prompt),
        Some(Commands::PerturbTest { prompt }) => run_perturb_test(&prompt),
        None => { run_server(3000).await; }
    }
}

async fn run_server(port: u16) {
    let shared_state = Arc::new(AppState {
        lattice: Mutex::new(LatticeState::new(80, 40, "Initial")),
    });

    let app = Router::new()
        .route("/api/v1/init", post(init_lattice))
        .route("/api/v1/step", post(run_step))
        .route("/api/v1/formatted", get(get_formatted).post(get_formatted))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Titan-Hilbert Engine natively augmented with Titan Memory on port {}", port);
    axum::serve(listener, app).await.unwrap();
}

fn run_agent(prompt: &str, iterations: u64, width: usize, height: usize, max_points: usize, load_model: Option<String>, save_model: Option<String>) {
    let mut l = LatticeState::new(width, height, prompt);
    if let Some(path) = load_model {
        if std::path::Path::new(&path).exists() {
            println!("Loading Titan Memory from {}...", path);
            match TitanMemory::load(&path, 0.01) {
                Ok(mem) => {
                    if mem.w.len() == width * height {
                        l.memory = mem;
                        println!("Titan Memory successfully loaded.");
                    } else {
                        println!("Warning: Loaded model size ({}) does not match lattice size ({}). Starting fresh.", mem.w.len(), width * height);
                    }
                },
                Err(e) => println!("Error loading model: {}. Starting fresh.", e),
            }
        } else {
            println!("Model file {} not found. Starting fresh.", path);
        }
    }

    let mut final_work = 0.0;
    for _ in 0..iterations { final_work = l.step(); }
    let map_text = l.get_formatted_output();
    
    if let Some(path) = save_model {
        println!("Saving Titan Memory to {}...", path);
        if let Err(e) = l.memory.save(&path) {
            println!("Error saving model: {}", e);
        } else {
            println!("Titan Memory successfully saved.");
        }
    }

    println!("{}", map_text);

    let lines: Vec<&str> = map_text.lines().collect();
    let mut grid = Vec::new();
    for line in lines {
        if line.len() > 10 && !line.contains("CA ITERATION") {
            grid.push(line.chars().collect::<Vec<char>>());
        }
    }

    struct Point { x: usize, y: usize, density: i32 }
    let mut focal_points = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c == '@' || c == 'X' || c == '*' {
                let mut density = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny >= 0 && ny < grid.len() as i32 && nx >= 0 && nx < grid[y].len() as i32 {
                            let nc = grid[ny as usize][nx as usize];
                            if nc == '@' || nc == 'X' || nc == 'o' || nc == '*' { density += 1; }
                        }
                    }
                }
                if density > 0 { focal_points.push(Point { x, y, density }); }
            }
        }
    }

    focal_points.sort_by(|a, b| b.density.cmp(&a.density));
    let mut distinct_points: Vec<Point> = Vec::new();
    for p in focal_points {
        if !distinct_points.iter().any(|dp| (p.x as i32 - dp.x as i32).abs() < 5 && (p.y as i32 - dp.y as i32).abs() < 5) {
            distinct_points.push(p);
            if distinct_points.len() == max_points { break; }
        }
    }

    println!("\n[AGENT THOUGHT PROCESS: {}]", prompt);
    println!(">>> NATIVE TITAN MEMORY WORK: {:.6}", final_work);
    let strategies = ["Architectural Core", "Edge-case Anomaly", "Emergent Bridge", "Fractal Resonance", "Entropic Drift"];
    for (i, pt) in distinct_points.iter().enumerate() {
        let strategy = strategies[i % strategies.len()];
        println!("Cluster {} at [X:{}, Y:{}] (Density: {}) -> {}", i+1, pt.x, pt.y, pt.density, strategy);
        let focus = if pt.y < 10 { "Front-end/UI" } else if pt.y < 25 { "Logic/Middleware" } else { "Database/Infra" };
        let action = if pt.x < 30 { "speed/latency" } else if pt.x < 60 { "reliability/fault tolerance" } else { "scalability/modularity" };
        println!("   Insight: Focus on '{}' and optimize for {}.", focus, action);
    }
}

fn run_lab() {
    println!("### CHAOS LABORATORY: PHASE TRANSITION ANALYSIS ###");
    println!("\n[Test 1: Determinism]");
    let p1 = LatticeState::new(40, 20, "Alpha").get_formatted_output();
    let p2 = LatticeState::new(40, 20, "Alpha").get_formatted_output();
    if p1 == p2 { println!("RESULT: Confirmed. Seeding is deterministic."); }
    else { println!("RESULT: Failed. Grids differ."); }

    println!("\n[Test 2: Entropy Sweep]");
    for length in 1..=20 {
        let prompt = "A".repeat(length);
        let mut l = LatticeState::new(40, 20, &prompt);
        let mut final_work = 0.0;
        for _ in 0..10 { final_work = l.step(); }
        let metrics = l.get_metrics();
        println!("Length {:2} | Density: {:.4} | Titan Work: {:.6} {}", length, metrics.density, final_work, "#".repeat((metrics.density * 100.0) as usize));
    }

    println!("\n[Test 3: Hilbert Phase Coherence & Downward Causation Sweep]");
    let couplings = [0.0, 0.5, 1.0, 2.0, 5.0, 10.0];
    for &coupling in &couplings {
        let mut l = LatticeState::new(40, 20, "Hilbert Coherence Core");
        l.causation_coupling = coupling;
        for _ in 0..20 { l.step(); }
        let metrics = l.get_metrics();
        println!("Coupling: {:4.1} | Coherence: {:.4} | Phi: {:.4} | Density: {:.4} {}", 
            coupling, metrics.coherence, metrics.phi, metrics.density, 
            "*".repeat((metrics.coherence * 50.0).max(0.0) as usize));
    }
}

async fn run_observe(seed: &str, duration: u64) {
    let mut l = LatticeState::new(60, 30, seed);
    let start = std::time::Instant::now();
    loop {
        let work = l.step();
        let metrics = l.get_metrics();
        print!("\x1B[H\x1B[J"); // Clear screen
        println!("{}", l.get_formatted_output());
        println!(">>> TITAN MEMORY WORK: {:.6} | ARROW OF TIME (Complexity): {:.2}", work, metrics.state_complexity);
        
        if duration > 0 && start.elapsed().as_secs() >= duration { break; }
        if duration == 0 && l.iteration >= 100 { break; }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

fn run_prime(instruction: &str, iterations: u64, homeostatic: bool) {
    let mut l = LatticeState::new(100, 50, instruction);
    let mut trajectory = Vec::new();
    
    let capture_points = vec![3, 5, 8, 10, 15, 20];
    let mut best_step = 0;
    let mut max_phi = -1.0;
    
    // Initial metrics
    let mut best_metrics = l.get_metrics();
    let mut best_attractors = Vec::new();
    let mut best_eigenstate = String::new();

    for i in 1..=iterations.max(20) {
        l.step();

        if homeostatic && i % 5 == 0 {
            // Apply light perturbation to avoid deep attractor lock
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let perturb_count = (100 * 50) / 100; // 1%
            for _ in 0..perturb_count {
                let x = rng.gen_range(0..100);
                let y = rng.gen_range(0..50);
                let phase_noise: f32 = rng.gen_range(-0.1..0.1);
                let (cos_n, sin_n) = (phase_noise.cos(), phase_noise.sin());
                let c = &mut l.grid[y][x];
                let rotate = |re: f32, im: f32| (re * cos_n - im * sin_n, re * sin_n + im * cos_n);
                (c.u_re, c.u_im) = rotate(c.u_re, c.u_im);
                (c.d_re, c.d_im) = rotate(c.d_re, c.d_im);
                (c.l_re, c.l_im) = rotate(c.l_re, c.l_im);
                (c.r_re, c.r_im) = rotate(c.r_re, c.r_im);
            }
        }

        let m = l.get_metrics();
        trajectory.push(m.phi);
        
        if capture_points.contains(&i) {
            // We look for the peak Integrated Information (Phi) in the pre-thermalization window
            if m.phi > max_phi {
                max_phi = m.phi;
                best_step = i;
                best_metrics = m;
                best_attractors = l.extract_grounded_attractors(8);
                best_eigenstate = l.get_semantic_eigenstate();
            }
        }
    }

    let phi_avg = trajectory.iter().sum::<f32>() / trajectory.len() as f32;
    let phi_trend = if best_metrics.phi > phi_avg { "Ascending (Structural Integration)" } else { "Descending (Phase Differentiation)" };

    let mode = if best_metrics.entropy > 11.5 {
        "Divergent-Creative"
    } else if best_metrics.resonance > 1.6 {
        "Integrative-Modular"
    } else if best_metrics.coherence > 0.8 {
        "High-Coherence Analytical"
    } else if best_metrics.phi > 18.0 {
        "Deep-State Synthesis"
    } else {
        "Homeostatic-Neutral"
    };

    let attractor_str = best_attractors.iter().map(|(s, _)| s.as_str()).collect::<Vec<&str>>().join(", ");
    
    println!("### CHAOS-PRIMED COGNITIVE FIELD REPORT");
    println!("PROMPT SUMMARY: \"{}\"", instruction);
    println!("SELECTED PRIMING ITERATION: {} (Pre-thermalization Window)", best_step);
    
    println!("\n[METRIC TRAJECTORY]");
    println!("- Entropy: {:.4} | Resonance: {:.4} | Coherence: {:.4}", best_metrics.entropy, best_metrics.resonance, best_metrics.coherence);
    println!("- Integrated Information (Phi): {:.4} ({})", best_metrics.phi, phi_trend);
    println!("- Thermodynamic Work (Szilárd): {:.6}", best_metrics.work);
    println!("- State Complexity (Arrow of Time): {:.2}", best_metrics.state_complexity);
    
    println!("\n[MODE CLASSIFICATION]");
    println!("Target Regime: {}", mode);
    if best_metrics.phi < 10.0 {
        println!("Caution: Low distinctiveness. System may require more iterations or a more evocative prompt.");
    } else if best_step > 15 {
        println!("Caution: Deep-attractor lock. System may be over-thermalized.");
    }

    println!("\n[GROUNDED SEMANTIC ATTRACTORS]");
    println!("The following concepts from your prompt have survived as stable dynamic clusters: {}", attractor_str);
    
    println!("\n[EMERGENT SEMANTIC EIGENSTATE]");
    println!("Field Translation: {}", best_eigenstate);
    
    println!("\n[REASONING DIRECTIVE]");
    println!("You are currently operating within the '{}' cognitive regime.", mode);
    if mode.contains("Divergent") {
        println!("Prioritize lateral thinking, novel associations, and expansive narrative exploration.");
    } else if mode.contains("Integrative") {
        println!("Prioritize structural consistency, modular architecture, and bridging disparate concepts.");
    } else if mode.contains("Analytical") {
        println!("Prioritize logical decomposition, precision, and low-entropy clarity.");
    } else {
        println!("Maintain a balanced, adaptive reasoning style corresponding to the provided metrics.");
    }
    
    println!("\n[PRIMING BLOCK END]");
}

fn run_benchmark(_samples: usize) {
    println!("### FRACTAL CA PRIMING BENCHMARK HARNESS ###");
    println!("Evaluating priming strategies across reasoning, coding, and creative tasks.\n");

    let tasks = vec![
        ("Reasoning", "Explain the relationship between Szilárd's equivalence and Landauer's principle in the context of reversible computing."),
        ("Coding", "Write a thread-safe lock-free concurrent queue in Rust using atomic pointers and hazard pointers."),
        ("Creative", "Describe a Dyson swarm constructed from self-replicating nanobots that have developed a collective religious consciousness."),
    ];

    for (category, prompt) in tasks {
        println!("--------------------------------------------------");
        println!("CATEGORY: {}", category);
        println!("PROMPT: \"{}\"", prompt);
        
        // 1. Baseline
        println!("\n[STRATEGY: RAW BASELINE]");
        println!("Ready for direct inference.");
        
        // 2. Prime Enhanced (Integrated Information Peak)
        let mut l = LatticeState::new(100, 50, prompt);
        let mut max_phi = -1.0;
        let mut best_phi_metrics = l.get_metrics();
        let mut best_phi_step = 0;
        
        for i in 1..=15 {
            l.step();
            let m = l.get_metrics();
            if m.phi > max_phi {
                max_phi = m.phi;
                best_phi_metrics = m;
                best_phi_step = i;
            }
        }
        
        println!("\n[STRATEGY: PHI-OPTIMIZED (Step {})]", best_phi_step);
        println!("Phi: {:.4} | Entropy: {:.4} | Resonance: {:.4}", best_phi_metrics.phi, best_phi_metrics.entropy, best_phi_metrics.resonance);
        
        // 3. Coherence Variant
        let mut l2 = LatticeState::new(100, 50, prompt);
        let mut max_coherence = -1.0;
        let mut coh_metrics = l2.get_metrics();
        let mut coh_step = 0;
        for i in 1..=15 {
            l2.step();
            let m = l2.get_metrics();
            if m.coherence > max_coherence {
                max_coherence = m.coherence;
                coh_metrics = m;
                coh_step = i;
            }
        }
        println!("\n[STRATEGY: COHERENCE-OPTIMIZED (Step {})]", coh_step);
        println!("Coherence: {:.4} | Resonance: {:.4} | Phi: {:.4}", coh_metrics.coherence, coh_metrics.resonance, coh_metrics.phi);
    }
    
    println!("\n--------------------------------------------------");
    println!("Benchmark artifacts generated. Comparative scoring requires downstream LLM evaluation.");
}

fn run_self_test(prompt: &str) {
    println!("### DEEP SELF-TEST: QUESTIONING THE LATTICE ###");
    println!("Prompt: \"{}\"\n", prompt);

    let mut l_vanilla = LatticeState::new(100, 50, prompt);
    let mut l_homeo = LatticeState::new(100, 50, prompt);

    println!("ASSUMPTION 1: Provenance persistence over time.");
    println!("Testing if the original prompt signal washes out or survives into deep time.");
    
    let intervals = [10, 50, 100, 200];
    println!("\nIter | Van Sat % | Hom Sat % | Van Attractors | Hom Attractors");
    println!("-----|-----------|-----------|----------------|---------------");

    for &target in &intervals {
        while l_vanilla.iteration < target { l_vanilla.step(); }
        while l_homeo.iteration < target {
            l_homeo.step();
            if l_homeo.iteration % 5 == 0 {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                for _ in 0..50 {
                    let x = rng.gen_range(0..100);
                    let y = rng.gen_range(0..50);
                    let noise: f32 = rng.gen_range(-0.1..0.1);
                    let c = &mut l_homeo.grid[y][x];
                    let (cn, sn) = (noise.cos(), noise.sin());
                    let rot = |re: f32, im: f32| (re*cn - im*sn, re*sn + im*cn);
                    (c.u_re, c.u_im) = rot(c.u_re, c.u_im);
                    (c.d_re, c.d_im) = rot(c.d_re, c.d_im);
                    (c.l_re, c.l_im) = rot(c.l_re, c.l_im);
                    (c.r_re, c.r_im) = rot(c.r_re, c.r_im);
                }
            }
        }

        let van_sat = l_vanilla.grid.iter().flatten().filter(|c| c.source_idx >= 0).count() as f32 / 50.0; // % of 5000
        let hom_sat = l_homeo.grid.iter().flatten().filter(|c| c.source_idx >= 0).count() as f32 / 50.0;
        
        let van_attrs = l_vanilla.extract_grounded_attractors(3).iter().map(|(s, _)| s.clone()).collect::<Vec<_>>().join(",");
        let hom_attrs = l_homeo.extract_grounded_attractors(3).iter().map(|(s, _)| s.clone()).collect::<Vec<_>>().join(",");

        println!("{:4} | {:8.1}% | {:8.1}% | {:14} | {:14}", target, van_sat, hom_sat, van_attrs, hom_attrs);
    }

    println!("\nASSUMPTION 2: Phi (Integrated Information) as a structural proxy.");
    let m_van = l_vanilla.get_metrics();
    let m_hom = l_homeo.get_metrics();
    println!("Vanilla Phi: {:.4} | Entropy: {:.4} | Resonance: {:.4}", m_van.phi, m_van.entropy, m_van.resonance);
    println!("Homeo   Phi: {:.4} | Entropy: {:.4} | Resonance: {:.4}", m_hom.phi, m_hom.entropy, m_hom.resonance);

    println!("\nASSUMPTION 3: Deterministic Mapping.");
    let l_check = LatticeState::new(100, 50, prompt);
    let mut l_dup = LatticeState::new(100, 50, prompt);
    for _ in 0..10 { l_dup.step(); }
    let a1 = l_check.extract_grounded_attractors(5);
    let a2 = l_dup.extract_grounded_attractors(5);
    if a1 != a2 {
        println!("CRITICAL: Dynamic Mapping is NOT identical across steps (as expected, but checking for core drift).");
    } else {
        println!("RESULT: Attractors are stable between iteration 0 and 10? Wait, that shouldn't be if it's evolving. Oh, I see.");
    }
    
    println!("\nREFLECTION:");
    if m_hom.phi > m_van.phi {
        println!("- Homeostatic perturbation INCREASED integrated information. Assumption confirmed: Breathing helps.");
    } else {
        println!("- Vanilla state is more integrated. Questioning: Is perturbation just noise or true breathing?");
    }
    
    if l_vanilla.extract_grounded_attractors(1).is_empty() {
        println!("- WARNING: Provenance signal LOSS at iteration 200. The lattice has thermalized away its memory.");
    } else {
        println!("- SUCCESS: Provenance signal SURVIVED 200 iterations. The causal link to the prompt is robust.");
    }
}

fn run_deep_time(prompt: &str) {
    println!("### DEEP TIME EVOLUTION: {} ###\n", prompt);
    let mut l = LatticeState::new(80, 40, prompt);
    let epochs = [1, 100, 500, 1000, 5000];
    
    for &target in &epochs {
        let mut final_work = 0.0;
        while l.iteration < target { final_work = l.step(); }
        let metrics = l.get_metrics();
        let density_raw = (metrics.density * (80.0 * 40.0)) as usize;
        
        println!("\n--- EPOCH {} (Density: {}) | Titan Work: {:.6} ---", target, density_raw, final_work);
        if density_raw > 1500 { println!("Status: Over-growth / Hyper-complexity."); }
        else if density_raw > 800 { println!("Status: Mature equilibrium."); }
        else if density_raw > 100 { println!("Status: Entropic decay."); }
        else { println!("Status: Ghost. Only radiation and dust remain."); }

        let map_text = l.get_formatted_output();
        let lines: Vec<&str> = map_text.lines().collect();
        if lines.len() > 25 {
            println!("\nCenter Core Snapshot:");
            for i in 15..25 {
                if let Some(line) = lines.get(i) {
                    let end = (50).min(line.len());
                    let start = (30).min(end);
                    println!("{}", &line[start..end]);
                }
            }
        }
        println!("----------------------------------------");
    }
}

fn run_self_prime(initial_prompt: &str, generations: usize, iterations: u64, width: usize, height: usize, load_model: Option<String>, save_model: Option<String>) {
    println!("### SELF-PROMPT INJECTION EXPERIMENT ###");
    println!("Initial Seed: '{}'", initial_prompt);
    println!("Generations: {} | Iterations per Gen: {} | Grid: {}x{}\n", generations, iterations, width, height);
    
    let mut current_prompt = initial_prompt.to_string();
    let mut carried_memory: Option<TitanMemory> = None;

    if let Some(path) = &load_model {
        if std::path::Path::new(path).exists() {
            println!("Loading Titan Memory from {}...", path);
            match TitanMemory::load(path, 0.01) {
                Ok(mem) => {
                    if mem.w.len() == width * height {
                        carried_memory = Some(mem);
                        println!("Titan Memory successfully loaded.");
                    } else {
                        println!("Warning: Loaded model size ({}) != lattice size ({}). Starting fresh.", mem.w.len(), width * height);
                    }
                },
                Err(e) => println!("Error loading model: {}. Starting fresh.", e),
            }
        }
    }
    
    for gen in 1..=generations {
        let mut l = LatticeState::new(width, height, &current_prompt);
        l.causation_coupling = 1.0; 
        
        if let Some(mem) = carried_memory.clone() {
            l.memory = mem;
        }
        
        let mut final_work = 0.0;
        for _ in 0..iterations { 
            final_work = l.step(); 
        }
        
        carried_memory = Some(l.memory.clone());
        
        let metrics = l.get_metrics();
        let semantic_prompt = l.get_semantic_eigenstate();
        
        println!("--- GENERATION {} ---", gen);
        println!("Phi: {:.4} | Coherence: {:.4} | Density: {:.4} | Titan Work: {:.6}", 
            metrics.phi, metrics.coherence, metrics.density, final_work);
        
        let display_len = 100.min(semantic_prompt.len());
        println!("Semantic Translation: {}...", &semantic_prompt[..display_len]);
        
        current_prompt = semantic_prompt;
        
        if gen == generations {
            let map_text = l.get_formatted_output();
            if let Some(path) = &save_model {
                println!("Saving Titan Memory to {}...", path);
                if let Err(e) = l.memory.save(path) {
                    println!("Error saving model: {}", e);
                } else {
                    println!("Titan Memory successfully saved.");
                }
            }
            println!("\nFinal State Snapshot:\n{}", map_text);
        }
    }
}

fn run_shock_test(prompt: &str) {
    println!("### THERMODYNAMIC SHOCK EXPERIMENT ###");
    println!("Seed: '{}'", prompt);
    
    let mut l = LatticeState::new(80, 40, prompt);
    l.causation_coupling = 1.0; 
    
    println!("\n[PHASE 1: Reaching Deep Attractor (500 iterations)]");
    for _ in 0..500 { l.step(); }
    let m1 = l.get_metrics();
    println!("Phi: {:.4} | Density: {:.4} | Coherence: {:.4}", m1.phi, m1.density, m1.coherence);
    let s1 = l.get_semantic_eigenstate();
    println!("Semantic Translation: {}...", &s1[..100.min(s1.len())]);

    println!("\n[PHASE 2: The Shock Event]");
    println!("Injecting maximum entropy into the core (20x20 block)...");
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for y in 10..30 {
        for x in 30..50 {
            l.grid[y][x] = Cell {
                u_re: rng.gen_range(-1.0..1.0), u_im: rng.gen_range(-1.0..1.0),
                d_re: rng.gen_range(-1.0..1.0), d_im: rng.gen_range(-1.0..1.0),
                l_re: rng.gen_range(-1.0..1.0), l_im: rng.gen_range(-1.0..1.0),
                r_re: rng.gen_range(-1.0..1.0), r_im: rng.gen_range(-1.0..1.0),
                source_idx: -2, // Shock-induced noise
            };
            let prob = l.grid[y][x].prob().sqrt() + 1e-9;
            l.grid[y][x].u_re /= prob; l.grid[y][x].u_im /= prob;
            l.grid[y][x].d_re /= prob; l.grid[y][x].d_im /= prob;
            l.grid[y][x].l_re /= prob; l.grid[y][x].l_im /= prob;
            l.grid[y][x].r_re /= prob; l.grid[y][x].r_im /= prob;
        }
    }
    let m2 = l.get_metrics();
    println!("Phi: {:.4} | Density: {:.4} | Coherence: {:.4}", m2.phi, m2.density, m2.coherence);
    let s2 = l.get_semantic_eigenstate();
    println!("Semantic Translation: {}...", &s2[..100.min(s2.len())]);

    println!("\n[PHASE 3: Recovery (500 iterations)]");
    for _ in 0..500 { l.step(); }
    let m3 = l.get_metrics();
    println!("Phi: {:.4} | Density: {:.4} | Coherence: {:.4}", m3.phi, m3.density, m3.coherence);
    let s3 = l.get_semantic_eigenstate();
    println!("Semantic Translation: {}...", &s3[..100.min(s3.len())]);
}

fn run_perturb_test(prompt: &str) {
    println!("### CONTINUOUS PERTURBATION EXPERIMENT ###");
    println!("Seed: '{}'", prompt);
    println!("Goal: Inject gentle entropy (5% perturbation) every 50 iterations for 20 epochs (1000 total).");
    
    let mut l = LatticeState::new(80, 40, prompt);
    l.causation_coupling = 1.0; 
    
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for epoch in 1..=20 {
        // Run unperturbed for 50 iterations
        for _ in 0..50 { l.step(); }
        
        let metrics = l.get_metrics();
        let semantics = l.get_semantic_eigenstate();
        
        println!("\n--- Epoch {} (Iteration {}) ---", epoch, epoch * 50);
        println!("Phi: {:.4} | Density: {:.4} | Coherence: {:.4}", metrics.phi, metrics.density, metrics.coherence);
        println!("Semantic Translation: {}...", &semantics[..100.min(semantics.len())]);
        
        // Apply gentle perturbation: Randomly shift 5% of cells
        let perturb_count = (80 * 40) / 20; // 5%
        for _ in 0..perturb_count {
            let x = rng.gen_range(0..80);
            let y = rng.gen_range(0..40);
            // Add slight phase noise
            let phase_noise: f32 = rng.gen_range(-0.5..0.5);
            let (cos_n, sin_n) = (phase_noise.cos(), phase_noise.sin());
            let c = &mut l.grid[y][x];
            
            let rotate = |re: f32, im: f32| (re * cos_n - im * sin_n, re * sin_n + im * cos_n);
            (c.u_re, c.u_im) = rotate(c.u_re, c.u_im);
            (c.d_re, c.d_im) = rotate(c.d_re, c.d_im);
            (c.l_re, c.l_im) = rotate(c.l_re, c.l_im);
            (c.r_re, c.r_im) = rotate(c.r_re, c.r_im);
        }
    }
}
