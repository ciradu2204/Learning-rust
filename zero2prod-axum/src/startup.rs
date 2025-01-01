use std::{net::TcpListener, sync::Arc};

use axum::{ routing::{get, post}, Router};
use axum_server::Server;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};
 pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<impl std::future::Future<Output = Result<(), std::io::Error>>, std::io::Error>{
     let db_pool = Arc::new(db_pool);
    let app: Router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool);
     
 
    let server = Server::from_tcp(listener).serve(app.into_make_service());
    Ok(server)
}