mod api;
mod app;
mod db;
mod logging;
mod models;
mod schema;

use dotenvy::dotenv;
use poem::{EndpointExt, listener::TcpListener, middleware};

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    dotenv().ok();
    logging::init_logging();

    let connection_pool = db::pool::create_db_connection_pool();
    let app = app::init_app(SERVER_PORT)
        .with(middleware::AddData::new(connection_pool))
        .with(middleware::Tracing)
        .with(middleware::CatchPanic::new());

    poem::Server::new(TcpListener::bind(format!(
        "{}:{}",
        SERVER_ADDR, SERVER_PORT
    )))
    .run(app)
    .await
    .expect(
        format!(
            "Failed to start server at http://{}:{}",
            SERVER_ADDR, SERVER_PORT
        )
        .as_str(),
    );
}
