use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

struct AppState {
    pool: MySqlPool,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/callback")]
async fn callback() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(callback)
            .app_data(AppState { pool: pool.clone() })
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
