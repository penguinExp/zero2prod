use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]

async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Unable to bind to 8080 port");

    run(listener)?.await
}
