use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_server::Server;
use std::net::TcpListener;


async fn greet(req: Request) -> impl IntoResponse {
    // let new_name = if name.is_empty() { "World" } else { &name };
    let path = req.uri().path(); 
    let  name = if path == "/"{
         "World"
    }else{
         path.trim_start_matches("/")
    };

    format!("Hello, {}!", name)
}

async fn health_check() -> impl IntoResponse{
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> Result<impl std::future::Future<Output = Result<(), std::io::Error>>, std::io::Error>{
    let app: Router = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check));
    
 
    let server = Server::from_tcp(listener).serve(app.into_make_service());
    Ok(server)
}
