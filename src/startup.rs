use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use routes::{health_check, subscribe};

use crate::routes;
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// Test actions
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    //No.await here!
    Ok(server)
}
