use std::net::TcpListener;

use user_map_service::startup::run;
use aws_sdk_dynamodb::{Client};

// whenever you have an async executor
// the thing it will act on is a future tree
// which includes a root future
// starts in root future
// context switch to something else it can
// do.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // because i dont have any other futures
    // its sync
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config); // this needs to implement FromRequest trait

    let address = format!("127.0.0.1:{}", 8080);
    let listener = TcpListener::bind(address)?;
    // this awaits the returned "Impl Future" in run
    // top of the future tree 
    run(listener, client)?.await
}
