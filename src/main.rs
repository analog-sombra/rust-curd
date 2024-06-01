mod config;
mod error;
mod model;

use axum::{routing::get, serve, Json, Router};
use simple_logger::SimpleLogger;

use crate::model::index::home::HomePage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let config = (*config::config::CONFIG).clone();
    println!("{:?}", config);

    let homepage: HomePage = HomePage {
        name: config.name,
        version: config.version,
        authour: config.author,
        github: config.github,
    };

    let routes_hello: Router = Router::new().route("/", get(|| async { Json(homepage) }));

    let url = format!("{}:{}", config.server, config.port);

    let listener = tokio::net::TcpListener::bind(url.clone()).await.unwrap();
    log::info!("Listning: http://{}/", url);

    serve(listener, routes_hello).await.unwrap();

    Ok(())
}
