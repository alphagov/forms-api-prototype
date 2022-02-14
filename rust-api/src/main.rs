extern crate dotenv;

use color_eyre::eyre::Result;
use color_eyre::Report;
use dotenv::dotenv;
use poem::middleware::Cors;
use poem::EndpointExt;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPool;

mod api;

fn setup() -> Result<(), Report> {
    dotenv().ok();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;
    let pool = PgPool::connect("postgres://postgres:password@localhost/postgres").await?;

    let api_service = OpenApiService::new(api::Api, "Forms API Rust Prototype", "0.0.1-alpha")
        .server("http://0.0.0.0:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .data(pool)
                .with(Cors::new()),
        )
        .await?;
    Ok(())
}
