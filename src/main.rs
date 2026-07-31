mod ffi;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, FromRow)]
pub struct Entry {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub nsecs: i64,
}

#[derive(Debug, Deserialize)]
pub struct пейлоад {
    pub причина: String,
    pub базар: String,
}

pub enum AppError {
    Db(sqlx::Error),
    NotFound,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Db(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "не найдено").into_response(),
            AppError::Db(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("база сломалась: {e}"),
            )
                .into_response(),
        }
    }
}

fn format_its() -> String {
    const ITS_DAY_NS: u64 = 86_400_000_000_000; // 86400 * 1e9
    const ITS_YEAR_DAYS: u64 = 147;
    const ITS_MONTH_DAYS: u64 = 21;

    let delta_ns = ffi::current_nsecs();
    let days = &delta_ns / ITS_DAY_NS;
    let rem_ns = &delta_ns % ITS_DAY_NS;

    let years = &days / ITS_YEAR_DAYS;
    let rem_days = &days % ITS_YEAR_DAYS;
    let months = &rem_days / ITS_MONTH_DAYS;
    let day_of_month = &rem_days % ITS_MONTH_DAYS;

    let secs = rem_ns / 1_000_000_000u64;
    let hours = &secs / 3600u64;
    let minutes = (&secs % 3600u64) / 60u64;
    let seconds = &secs % 60u64;

    return format!("{years}y {months}m {day_of_month}d {hours:02}:{minutes:02}:{seconds:02}")
}

async fn хрень_get(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Entry>>, AppError> {
    let stacy = sqlx::query_as::<_, Entry>("SELECT id, reazon AS title, bazar AS content, COALESCE(time, 0) AS nsecs FROM entrys")
        .fetch_all(&pool)
        .await?;
    return Ok(Json(stacy))
}

async fn хрень_post(
    State(pool): State<SqlitePool>,
    Json(payload): Json<пейлоад>,
) -> Result<Json<Entry>, AppError> {
    let now: i64 = ffi::current_nsecs().try_into().unwrap_or(i64::MAX);
    let goddamn = sqlx::query_as::<_, Entry>("INSERT INTO entrys (reazon, bazar, time) VALUES (?, ?, ?) RETURNING id, reazon AS title, bazar AS content, COALESCE(time, 0) AS nsecs")
    .bind(&payload.причина)
    .bind(&payload.базар)
    .bind(now)
    .fetch_one(&pool)
    .await?;
    return Ok(Json(goddamn))
}

async fn хрень_single_get(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Entry>, AppError> {
    let 鬱病 = sqlx::query_as::<_, Entry>
    ("SELECT id, reazon AS title, bazar AS content,
    COALESCE(time, 0) AS nsecs FROM entrys WHERE id = ?")
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    return Ok(Json(鬱病))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let its = format_its();
    println!("current nsecs: {its}");

    let options = SqliteConnectOptions::from_str("sqlite:fuck.db")?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("fuck migrations");

    let app = Router::new()
        .route("/api/%D1%85%D1%80%D0%B5%D0%BD%D1%8C", get(хрень_get).post(хрень_post))
        .route("/api/%D1%85%D1%80%D0%B5%D0%BD%D1%8C/{id}", get(хрень_single_get))
        .fallback_service(ServeDir::new("static/"))     // сомнительная хрень(надо делать / -> ServeDir())
        .with_state(pool);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await?,
        app)
    .await?;

    Ok(())
}
