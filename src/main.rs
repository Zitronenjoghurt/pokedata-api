use axum::Router;
use bincode;
use pokedata_api_types::app_state::AppState;
use std::io;
use std::sync::Arc;

const STATIC_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));

#[tokio::main]
async fn main() -> io::Result<()> {
    let app_state: AppState = bincode::deserialize(STATIC_DATA).unwrap();
    let shared_state = Arc::new(app_state);

    let app = Router::new()
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}