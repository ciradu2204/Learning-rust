use std::net::TcpListener;

 use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
     // Wrap the connection in a smart pointer
     let connection = web::Data::new(db_pool);
     // Capture `connection` from the surrounding environment
     let server = HttpServer::new(move || {
         App::new()
             .wrap(TracingLogger::default())
             .route("/health_check", web::get().to(health_check))
             .route("/subscriptions", web::post().to(subscribe))
             // Get a pointer copy and attach it to the application state
             .app_data(connection.clone())
     })
    .listen(listener)?
    .run();
    
    Ok(server)
}