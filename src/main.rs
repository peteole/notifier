mod config;
mod server;
mod services;
#[tokio::main]
async fn main() {
    let config = config::ConfigFile::load("config.yaml").unwrap();
    let config = config.to_config().await;
    let app = server::create_router(config);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
