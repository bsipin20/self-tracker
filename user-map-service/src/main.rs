use std::net::TcpListener;

use user_map_service::startup::run; 

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:{}", 8080);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
