use std::net::TcpListener;

use newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = String::from("127.0.0.1:0");
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
