extern crate dotenv;

use color_eyre::eyre::Result;
use color_eyre::Report;
use dotenv::dotenv;
use poem::middleware::Cors;
use poem::EndpointExt;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPool;
use tracing_subscriber::EnvFilter;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
mod api;
mod forms;

const SERVER_KEY: &[u8] = b"123456";
type ServerKey = Hmac<Sha256>;

fn setup() -> Result<(), Report> {
    dotenv().ok();
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;

    let pool = PgPool::connect("postgres://postgres:password@localhost/postgres").await?;

    let api_service = OpenApiService::new(
        api::Api, 
        "Forms API Rust Prototype", 
        env!("CARGO_PKG_VERSION"))
        .server("http://0.0.0.0:3000/api");
    let ui = api_service.swagger_ui();
    let server_key = ServerKey::new_from_slice(SERVER_KEY).expect("valid server key");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .data(pool)
                .data(server_key)
                .with(Cors::new()),
        )
        .await?;
    Ok(())
}
