mod config;
mod server;
mod services;
#[tokio::main]
async fn main() {
    let config_file = config::ConfigFile::load_from_std_locations();
    let config = config_file.to_config().await;
    let app = server::create_router(config);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
