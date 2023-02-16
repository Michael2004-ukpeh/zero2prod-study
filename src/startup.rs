use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes;
// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
// Test actions
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    //No.await here!
    Ok(server)
}
