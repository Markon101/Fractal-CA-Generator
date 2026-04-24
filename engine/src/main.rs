use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tracing_subscriber;

#[derive(Serialize, Deserialize, Clone)]
struct LatticeState {
    grid: Vec<Vec<u32>>, // Token IDs
    width: usize,
    height: usize,
    iteration: u64,
    seed_prompt: String,
    instruction_header: String,
}

#[derive(Deserialize)]
struct InitRequest {
    width: usize,
    height: usize,
    seed_prompt: String,
    instruction_header: Option<String>,
}

struct AppState {
    lattice: RwLock<LatticeState>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let initial_lattice = LatticeState {
        grid: vec![vec![0; 32]; 32],
        width: 32,
        height: 32,
        iteration: 0,
        seed_prompt: "Initial chaos".to_string(),
        instruction_header: "## CHAOTIC MAP INSTRUCTIONS\nObserve the emergent patterns below. Use the density of tokens to determine semantic intensity.".to_string(),
    };

    let shared_state = Arc::new(AppState {
        lattice: RwLock::new(initial_lattice),
    });

    let app = Router::new()
        .route("/api/v1/lattice/state", get(get_state))
        .route("/api/v1/lattice/init", post(init_lattice))
        .route("/api/v1/lattice/formatted", get(get_formatted_output))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Fractal CA Engine running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_state(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> Json<LatticeState> {
    let lattice = state.lattice.read().unwrap();
    Json(lattice.clone())
}

async fn get_formatted_output(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> String {
    let lattice = state.lattice.read().unwrap();
    let mut output = format!("{}\n\nPROMPT CONTEXT: {}\nITERATION: {}\n\n", 
        lattice.instruction_header, 
        lattice.seed_prompt,
        lattice.iteration
    );
    
    for row in &lattice.grid {
        for cell in row {
            // Simple mapping for visual "chaos" representation
            let char = match cell {
                0 => '.',
                1..=10 => '*',
                11..=50 => '#',
                51..=100 => '@',
                _ => '?',
            };
            output.push(char);
        }
        output.push('\n');
    }
    output
}

async fn init_lattice(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<InitRequest>,
) -> Json<LatticeState> {
    let mut lattice = state.lattice.write().unwrap();
    lattice.width = payload.width;
    lattice.height = payload.height;
    lattice.seed_prompt = payload.seed_prompt.clone();
    
    if let Some(h) = payload.instruction_header {
        lattice.instruction_header = h;
    }

    // Seed the grid with basic pattern derived from prompt length
    let seed_val = payload.seed_prompt.len() as u32;
    lattice.grid = vec![vec![0; payload.width]; payload.height];
    // Intersperse the seed
    for i in 0..payload.height {
        for j in 0..payload.width {
            if (i + j) % 7 == 0 {
                lattice.grid[i][j] = seed_val % 100;
            }
        }
    }
    
    lattice.iteration = 0;
    Json(lattice.clone())
}
