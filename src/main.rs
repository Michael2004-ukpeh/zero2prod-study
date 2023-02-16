use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if cant read config
    let configuration = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Tcp Listener
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}
