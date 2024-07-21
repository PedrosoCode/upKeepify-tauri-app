use sqlx::postgres::PgPoolOptions;
use tauri::command;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Ping {
    id: i32,
    value: String,
}

#[command]
async fn get_pings() -> Result<Vec<Ping>, String> {
    dotenv::dotenv().ok(); // Carrega o arquivo .env
    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to the database: {}", e))?;
    
    let pings = sqlx::query_as!(
        Ping,
        r#"SELECT id, value FROM ping"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to fetch pings: {}", e))?;

    Ok(pings)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_pings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
