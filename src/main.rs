use std::net::TcpListener;

use newsletter::config::get_config;
use newsletter::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
