use axum::{
    extract::{State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
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
}

#[derive(Serialize, Debug)]
struct Metrics {
    entropy: f32,
    density: f32,
    resonance: f32,
    phi: f32,   // Integrated Information Potential
    work: f32,  // Thermodynamic work (Szilárd equivalence)
    raw_sum: f32,
}

impl LatticeState {
    fn new(width: usize, height: usize, prompt: &str) -> Self {
        let mut grid = vec![vec![Cell::new(); width]; height];
        let mut semantic_field = vec![0.0; width * height];
        let bytes = prompt.as_bytes();
        
        if bytes.is_empty() {
            grid[height/2][width/2].u_re = 1.0;
        } else {
            for (i, &b) in bytes.iter().enumerate() {
                let mut found = false;
                for attempt in 0..100 {
                    let idx = (i * 13 + attempt * 7) % (width * height);
                    let x = idx % width;
                    let y = idx / width;
                    
                    if (x & y) == 0 || (x | y) % 3 == 0 {
                        let phase = (b as f32) / 255.0 * std::f32::consts::TAU;
                        grid[y][x].u_re = phase.cos() * 0.7;
                        grid[y][x].u_im = phase.sin() * 0.7;
                        grid[y][x].d_re = phase.cos() * 0.3;
                        grid[y][x].d_im = -phase.sin() * 0.3;
                        
                        semantic_field[y * width + x] = (b as f32) / 255.0;
                        found = true;
                        break;
                    }
                }
                
                if !found {
                    let x = (i * 7 + (b as usize) * 3) % width;
                    let y = (i * 5 + (b as usize) * 11) % height;
                    let phase = (b as f32) / 255.0 * std::f32::consts::TAU;
                    grid[y][x].u_re = phase.cos() * 0.5;
                    grid[y][x].u_im = phase.sin() * 0.5;
                    semantic_field[y * width + x] = (b as f32) / 255.0;
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
        }
    }

    fn step(&mut self) -> f32 {
        let mut next_grid = self.grid.clone();
        
        // 1. Hyper-dimensional feedback: Get modulation from Titan Memory based on previous state
        let (work, modulation_field) = self.memory.update_and_modulate(&self.current_probs, &self.current_probs); // Predict self
        
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

                let mut cell = Cell { u_re, u_im, d_re, d_im, l_re, l_im, r_re, r_im };
                
                let p = cell.prob();
                let idx = y * self.width + x;
                let mod_val = modulation_field[idx];
                let sem_val = self.semantic_field[idx];
                
                // Hyper-dimensional rotation: Driven by semantic prompt and Neural Memory
                let theta = p * 10.0 * (1.0 + mod_val) * (1.0 + sem_val * 2.0); 
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
            return Metrics { entropy: 0.0, density: 0.0, resonance: 0.0, phi: 0.0, work: 0.0, raw_sum: 0.0 };
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

        Metrics {
            entropy,
            density,
            resonance,
            phi,
            work,
            raw_sum: total_p,
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
    },
    Lab {},
    Observe {
        seed: String,
        #[arg(short, long, default_value_t = 0)] duration: u64,
    },
    Prime {
        instruction: String,
        #[arg(short, long, default_value_t = 10)] iterations: u64,
    },
    DeepTime {
        #[arg(default_value = "Europa Orbital Research Station: Thousand Year Legacy")] prompt: String,
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
        Some(Commands::Agent { prompt, iterations, width, height, points }) => run_agent(&prompt, iterations, width, height, points),
        Some(Commands::Lab {}) => run_lab(),
        Some(Commands::Observe { seed, duration }) => run_observe(&seed, duration).await,
        Some(Commands::Prime { instruction, iterations }) => run_prime(&instruction, iterations),
        Some(Commands::DeepTime { prompt }) => run_deep_time(&prompt),
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

fn run_agent(prompt: &str, iterations: u64, width: usize, height: usize, max_points: usize) {
    let mut l = LatticeState::new(width, height, prompt);
    let mut final_work = 0.0;
    for _ in 0..iterations { final_work = l.step(); }
    let map_text = l.get_formatted_output();
    println!("{}", map_text);

    let lines: Vec<&str> = map_text.lines().collect();
    let mut grid = Vec::new();
    for line in lines {
        if line.len() > 10 && !line.contains("CA ITERATION") {
            grid.push(line.chars().collect::<Vec<char>>());
        }
    }

    struct Point { x: usize, y: usize, density: i32, char: char }
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
                if density > 0 { focal_points.push(Point { x, y, density, char: c }); }
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
}

async fn run_observe(seed: &str, duration: u64) {
    let mut l = LatticeState::new(60, 30, seed);
    let start = std::time::Instant::now();
    loop {
        let work = l.step();
        print!("\x1B[H\x1B[J"); // Clear screen
        println!("{}", l.get_formatted_output());
        println!(">>> TITAN MEMORY WORK: {:.6}", work);
        
        if duration > 0 && start.elapsed().as_secs() >= duration { break; }
        if duration == 0 && l.iteration >= 100 { break; }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

fn run_prime(instruction: &str, iterations: u64) {
    let mut l = LatticeState::new(100, 50, instruction);
    let mut phi_trajectory = Vec::new();
    
    for _ in 0..iterations { 
        l.step(); 
        phi_trajectory.push(l.get_metrics().phi);
    }
    
    let metrics = l.get_metrics();
    let phi_avg = phi_trajectory.iter().sum::<f32>() / phi_trajectory.len() as f32;
    let phi_trend = if metrics.phi > phi_avg { "Ascending (Integrating)" } else { "Descending (Differentiating)" };

    let vibe = if metrics.entropy > 10.95 { "Highly divergent and creative." } else { "Convergent and analytical." };
    let structure = if metrics.resonance > 1.45 { "Strong rhythmic patterns detected. Use structured, modular responses." } else { "Diffuse state. Prefer fluid, narrative explanations." };

    let words: Vec<&str> = instruction.split_whitespace().collect();
    let mut rng = rand::thread_rng();
    let active_count = (words.len() as f32 * metrics.density * 5.0).max(1.0) as usize;
    let hotspots: Vec<&&str> = words.choose_multiple(&mut rng, active_count.min(words.len())).collect();
    let hotspot_str: String = hotspots.iter().map(|s| **s).collect::<Vec<&str>>().join(", ");

    println!("### CHAOS-PRIMED COGNITIVE FIELD");
    println!("[METRICS]\n- Entropy: {:.4} ({})\n- Resonance: {:.4} ({})\n- Integrated Information (Phi): {:.4} ({})", 
        metrics.entropy, vibe, metrics.resonance, structure, metrics.phi, phi_trend);
    println!("- Thermodynamic Work: {:.6} (Energy of Transition)", metrics.work);
    
    println!("\n[SEMANTIC ATTRACTOR]\nThe evolution converged on these core concepts: {}", hotspot_str);
    
    println!("\n[COGNITIVE DIRECTIVE]\nYou are operating within an integrated information field. Adjust your reasoning style to match the 'vibe' and 'structure' described above.");
    println!("If Phi is Ascending, prioritize synthesis and bridging concepts. If Descending, prioritize analysis and breakdown.");
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
