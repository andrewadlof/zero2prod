use zero2prod::run;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main()-> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `0000` - it's now coming fom our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener)?.await
}