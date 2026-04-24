use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Cell {
    u_re: f32, u_im: f32,
    d_re: f32, d_im: f32,
    l_re: f32, l_im: f32,
    r_re: f32, r_im: f32,
}

impl Cell {
    fn new() -> Self {
        Cell {
            u_re: 0.0, u_im: 0.0,
            d_re: 0.0, d_im: 0.0,
            l_re: 0.0, l_im: 0.0,
            r_re: 0.0, r_im: 0.0,
        }
    }
    fn prob(&self) -> f32 {
        self.u_re.powi(2) + self.u_im.powi(2) + self.d_re.powi(2) + self.d_im.powi(2) +
        self.l_re.powi(2) + self.l_im.powi(2) + self.r_re.powi(2) + self.r_im.powi(2)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct LatticeState {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    iteration: u64,
    seed_prompt: String,
    instruction_header: String,
    modulation_field: Vec<f32>, // Neural feedback from Titan
}

impl LatticeState {
    fn new(width: usize, height: usize, prompt: &str) -> Self {
        let mut grid = vec![vec![Cell::new(); width]; height];
        let bytes = prompt.as_bytes();
        
        if bytes.is_empty() {
            grid[height/2][width/2].u_re = 1.0;
        } else {
            // FRACTAL FOLDING: Inject bytes into a Sierpinski-like structure
            for (i, &b) in bytes.iter().enumerate() {
                // Find coordinates that fit a recursive bitwise pattern
                let mut found = false;
                for attempt in 0..100 {
                    let idx = (i * 13 + attempt * 7) % (width * height);
                    let x = idx % width;
                    let y = idx / width;
                    
                    // Simple bitwise check for "fractal-like" distribution (Sierpinski carpet/gasket variant)
                    if (x & y) == 0 || (x | y) % 3 == 0 {
                        let phase = (b as f32) / 255.0 * std::f32::consts::TAU;
                        grid[y][x].u_re = phase.cos() * 0.7;
                        grid[y][x].u_im = phase.sin() * 0.7;
                        grid[y][x].d_re = phase.cos() * 0.3;
                        grid[y][x].d_im = -phase.sin() * 0.3;
                        found = true;
                        break;
                    }
                }
                
                // Fallback if no fractal point found (unlikely but safe)
                if !found {
                    let x = (i * 7 + (b as usize) * 3) % width;
                    let y = (i * 5 + (b as usize) * 11) % height;
                    let phase = (b as f32) / 255.0 * std::f32::consts::TAU;
                    grid[y][x].u_re = phase.cos() * 0.5;
                    grid[y][x].u_im = phase.sin() * 0.5;
                }
            }
        }

        LatticeState {
            grid,
            width,
            height,
            iteration: 0,
            seed_prompt: prompt.to_string(),
            instruction_header: "## TITAN MEMORY CONTEXT".to_string(),
            modulation_field: vec![1.0; width * height],
        }
    }

    fn step(&mut self) {
        let mut next_grid = self.grid.clone();
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
                
                // TITAN MODULATION: Use the modulation field to warp the Hilbert space
                let p = cell.prob();
                let mod_val = self.modulation_field[y * self.width + x];
                let theta = p * 10.0 * mod_val; 
                let (cos_t, sin_t) = (theta.cos(), theta.sin());
                
                let rotate = |re: f32, im: f32| (re * cos_t - im * sin_t, re * sin_t + im * cos_t);
                (cell.u_re, cell.u_im) = rotate(cell.u_re, cell.u_im);
                (cell.d_re, cell.d_im) = rotate(cell.d_re, cell.d_im);
                (cell.l_re, cell.l_im) = rotate(cell.l_re, cell.l_im);
                (cell.r_re, cell.r_im) = rotate(cell.r_re, cell.r_im);

                next_grid[y][x] = cell;
            }
        }
        self.grid = next_grid;
        self.iteration += 1;
    }

    fn get_probs(&self) -> Vec<f32> {
        self.grid.iter().flatten().map(|c| c.prob()).collect()
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

struct AppState {
    lattice: Mutex<LatticeState>,
}

#[derive(Deserialize)]
struct InitRequest { 
    width: usize, 
    height: usize, 
    #[serde(alias = "seed_prompt")]
    seed: String,
    #[serde(default)]
    instruction_header: Option<String>,
}

async fn init_lattice(State(state): State<Arc<AppState>>, Json(payload): Json<InitRequest>) -> Json<String> {
    let mut l = state.lattice.lock().unwrap();
    *l = LatticeState::new(payload.width, payload.height, &payload.seed);
    if let Some(header) = payload.instruction_header {
        l.instruction_header = header;
    }
    Json("Init".to_string())
}

#[derive(Deserialize)]
struct ModulateRequest { 
    field: Option<Vec<f32>>,
    #[serde(default = "default_step_count")]
    count: u64,
}

fn default_step_count() -> u64 { 1 }

async fn run_step(State(state): State<Arc<AppState>>, Json(payload): Json<ModulateRequest>) -> Json<Vec<f32>> {
    let mut l = state.lattice.lock().unwrap();
    if let Some(field) = payload.field {
        if field.len() == l.modulation_field.len() {
            l.modulation_field = field;
        }
    }
    for _ in 0..payload.count {
        l.step();
    }
    Json(l.get_probs())
}

#[derive(Deserialize)]
struct FormattedRequest {
    #[serde(default)]
    steps: u64,
}

async fn get_formatted(State(state): State<Arc<AppState>>, payload: Option<Json<FormattedRequest>>) -> String {
    let mut l = state.lattice.lock().unwrap();
    if let Some(Json(p)) = payload {
        for _ in 0..p.steps {
            l.step();
        }
    }
    l.get_formatted_output()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let seed = &args[1];
        let iterations = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
        let width = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(120);
        let height = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(60);
        
        let mut l = LatticeState::new(width, height, seed);
        for _ in 0..iterations { l.step(); }
        println!("{}", l.get_formatted_output());
        return;
    }

    let shared_state = Arc::new(AppState {
        lattice: Mutex::new(LatticeState::new(80, 40, "Initial")),
    });

    let app = Router::new()
        .route("/api/v1/init", post(init_lattice))
        .route("/api/v1/step", post(run_step))
        .route("/api/v1/formatted", get(get_formatted).post(get_formatted))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Titan-Hilbert Engine on port 3000");
    axum::serve(listener, app).await.unwrap();
}
