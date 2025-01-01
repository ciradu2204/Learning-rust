 
use std::sync::Arc;

use axum::{ extract::{Form, State}, http::StatusCode, response::IntoResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use axum_macros::debug_handler;

 
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}
// Let's start simple: we always return a 200 OK

#[debug_handler] 
pub async fn subscribe(pool: State<Arc<PgPool>>, form: Form<FormData>) -> impl IntoResponse {

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
 }