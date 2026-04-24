use axum::{
    routing::{get, post},
    Json, Router,
};
use ndarray::Array2;
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
}

#[derive(Deserialize)]
struct InitRequest {
    width: usize,
    height: usize,
    seed_token: u32,
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
    };

    let shared_state = Arc::new(AppState {
        lattice: RwLock::new(initial_lattice),
    });

    let app = Router::new()
        .route("/api/v1/lattice/state", get(get_state))
        .route("/api/v1/lattice/init", post(init_lattice))
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

async fn init_lattice(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<InitRequest>,
) -> Json<LatticeState> {
    let mut lattice = state.lattice.write().unwrap();
    lattice.width = payload.width;
    lattice.height = payload.height;
    lattice.grid = vec![vec![payload.seed_token; payload.width]; payload.height];
    lattice.iteration = 0;
    Json(lattice.clone())
}
