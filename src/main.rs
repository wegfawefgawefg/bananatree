use crate::db::init_db;
use crate::models::{Banana, User};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::sync::Mutex;

mod db;
mod models;

async fn post_banana(
    db: web::Data<Mutex<tokio_postgres::Client>>,
    banana: web::Json<Banana>,
) -> impl Responder {
    let client = db.lock().unwrap();

    match client
        .execute(
            "INSERT INTO bananas (user_id, content) VALUES ($1, $2)",
            &[&banana.user_id, &banana.content], // Match these with your placeholders
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "Banana posted successfully"})),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to post banana"}))
        }
    }
}

async fn recent_bananas(db: web::Data<Mutex<tokio_postgres::Client>>) -> impl Responder {
    let client = db.lock().unwrap();

    match client
        .query(
            "SELECT id, user_id, content FROM bananas ORDER BY id DESC LIMIT 10",
            &[],
        )
        .await
    {
        Ok(rows) => {
            let bananas: Vec<Banana> = rows
                .into_iter()
                .map(|row| Banana {
                    id: Some(row.get("id")),
                    user_id: row.get("user_id"),
                    content: row.get("content"),
                })
                .collect();

            HttpResponse::Ok().json(bananas)
        }
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch bananas"}))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_client = init_db().await.expect("Failed to connect to the database");
    let db_client = web::Data::new(Mutex::new(db_client));

    HttpServer::new(move || {
        App::new()
            .app_data(db_client.clone())
            .route("/post_banana", web::post().to(post_banana))
            .route("/recent_bananas", web::get().to(recent_bananas))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
