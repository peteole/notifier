mod config;
mod server;
mod services;
use std::env;
use std::fs;

use utoipa::OpenApi;

const languages: &[&str] = &["rust", "go", "kotlin", "typescript-fetch", "python"];

fn main() {
    let docs = server::ApiDoc::openapi().to_pretty_json().unwrap();

    fs::write("./clients/openAPI.json", docs).unwrap();
    let client = testcontainers::clients::Cli::docker();
    let image = testcontainers::images::generic::GenericImage::new(
        "openapitools/openapi-generator-cli",
        "latest",
    )
    .with_volume(
        env::current_dir()
            .unwrap()
            .join("clients")
            .display()
            .to_string(),
        "/local",
    )
    .with_wait_for(testcontainers::core::WaitFor::StdOutMessage {
        message: "# Thanks for using OpenAPI Generator.".into(),
    });
    for language in languages {
        let run_image = testcontainers::RunnableImage::from((
            image.clone(),
            vec![
                "generate".to_string(),
                "-i".into(),
                "/local/openAPI.json".into(),
                "--git-user-id".into(),
                "peteole".into(),
                "--git-repo-id".into(),
                "notifier".into(),
                format!("--additional-properties=packageVersion={},packageName=clients/{},isGoSubmodule=true",env!("CARGO_PKG_VERSION"),language).into(),
                "-g".into(),
                language.to_string(),
                "-o".into(),
                format!("/local/{}", language),
            ],
        ));
        let _container = client.run(run_image);
    }
}
