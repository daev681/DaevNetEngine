mod engine;
mod net;
mod util;
mod config;
mod handler;

#[tokio::main]
async fn main() {
    engine::runtime::run_server().await;
}