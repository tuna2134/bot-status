use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod discord;
mod utils;

struct AppState {
    pool: MySqlPool,
    discord: discord::DiscordClient,
    ratelimit: Arc<Mutex<HashMap<String, utils::Ratelimit>>>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

#[get("/exchange_code")]
async fn callback(
    app_state: web::Data<AppState>,
    callback_query: web::Query<CallbackQuery>,
) -> impl Responder {
    let access_token = app_state
        .discord
        .exchange_code(callback_query.code.clone())
        .await
        .unwrap();
    // println!("{}", access_token.access_token);
    // let user: discord::DiscordUser = reqwest::
    // let is_registred = sqlx::query!("SELECT *")
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;
    let discord_client_id = env::var("DISCORD_CLIENT_ID")?;
    let discord_client_secret = env::var("DISCORD_CLIENT_SECRET")?;
    let discord_redirect_uri = env::var("DISCORD_REDIRECT_URI")?;
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(callback)
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
                discord: discord::DiscordClient::new(
                    discord_client_id.clone(),
                    discord_client_secret.clone(),
                    discord_redirect_uri.clone(),
                ),
                ratelimit: Arc::new(Mutex::new(HashMap::new())),
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
