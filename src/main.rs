use std::net::TcpListener;
use zero2prod::run;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Tcp Listener
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener)?.await
}
