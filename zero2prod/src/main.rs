use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run, telemetry::{get_subscriber, init_subscriber}};

 
#[tokio::main]
async fn main() -> std::io::Result<()> {
     let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
     init_subscriber(subscriber);
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("{}:{}", configuration.application.host,configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
