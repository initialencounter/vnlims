mod hander;

use axum::routing::{get, post, Router};
use axum_example_service::sea_orm::Database;
use hander::{
    create_project, delete_project, search_item_c_name, search_m_notes, search_projects,
    search_t_notes, static_handler, AppState, import_porjects,
};
use migration::{Migrator, MigratorTrait};
use std::env;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let app = Router::new()
        .route("/", get(static_handler))
        .route("/", post(create_project))
        .route("/search", get(search_projects))
        .route("/searchTNotes", get(search_t_notes))
        .route("/searchMNotes", get(search_m_notes))
        .route("/searchItemCName", get(search_item_c_name))
        .route("/import", get(import_porjects))
        .route("/delete/{id}", post(delete_project))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
