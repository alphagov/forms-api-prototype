use color_eyre::Report;
use color_eyre::eyre::Result;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{OpenApiService};
use poem::EndpointExt;
//use poem::middleware::Cors;
use sqlx::postgres::PgPoolOptions;

mod api;

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "0")
    }
    color_eyre::install()?;
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    };
    tracing_subscriber::fmt::init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;
    let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/postgres").await?;

    let api_service = OpenApiService::new(api::Api, "Forms API Rust Prototype", "0.0.1-alpha")
        .server("http://0.0.0.0:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui).data(pool))//.with(Cors))
        .await?;
    Ok(())
}
