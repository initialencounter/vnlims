mod handler;
mod layer;

use axum::{
    extract::ConnectInfo,
    routing::{get, post, Router},
};
use tracing_subscriber::fmt::time::FormatTime;
use axum_example_service::sea_orm::Database;
use handler::{
    create_project, delete_project, favicon_handler, get_table_update_time, import_porjects,
    search_count, search_item_c_name, search_m_notes, search_principal, search_projects,
    search_t_notes, static_handler, static_handler_404, AppState,
};
use layer::{decode_uri, resolve_ip_to_hostname};
use migration::{Migrator, MigratorTrait};
use std::{env, net::SocketAddr};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};

// 自定义时间格式化器，固定纳秒位数为9位
struct FixedNanosTime;

impl FormatTime for FixedNanosTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = time::OffsetDateTime::now_local()
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());

        // 手动格式化时间，固定纳秒为9位
        write!(
            w,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}{:+03}:{:02}",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            now.nanosecond(),
            now.offset().whole_hours(),
            now.offset().minutes_past_hour().abs()
        )
    }
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // 设置日志级别，过滤掉 sqlx 的 trace 日志
    env::set_var("RUST_LOG", "info,sqlx=warn");

    // 使用更灵活的日志初始化方式
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(FixedNanosTime)
        .with_target(false) // 不显示 target (axum_example_api)
        .init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许任何来源
        .allow_methods(Any) // 允许任何 HTTP 方法
        .allow_headers(Any); // 允许任何请求头

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(false))
        .on_request(
            |request: &hyper::Request<axum::body::Body>, _span: &tracing::Span| {
                if let Some(ConnectInfo(addr)) =
                    request.extensions().get::<ConnectInfo<SocketAddr>>()
                {
                    tracing::info!(
                        "{} [{}] {} {}",
                        resolve_ip_to_hostname(addr.ip()),
                        request.method(),
                        request.uri().path(),
                        decode_uri(request.uri().query().unwrap_or("")),
                    );
                }
            },
        )
        .on_response(
            |_response: &hyper::Response<axum::body::Body>,
             _latency: std::time::Duration,
             _span: &tracing::Span| {},
        );

    let app = Router::new()
        .route("/", get(static_handler))
        .route("/", post(create_project))
        .route("/favicon.ico", get(favicon_handler))
        .route("/search", get(search_projects))
        .route("/search_count", get(search_count))
        .route("/searchTNotes", get(search_t_notes))
        .route("/searchMNotes", get(search_m_notes))
        .route("/searchPrincipal", get(search_principal))
        .route("/searchItemCName", get(search_item_c_name))
        .route("/import", get(import_porjects))
        .route("/delete/{id}", post(delete_project))
        .route("/getLastUpdated", get(get_table_update_time))
        .fallback(static_handler_404)
        .layer(CompressionLayer::new())
        .layer(trace_layer)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
