use std::{cmp::min, env, sync::Arc};

use axum::{extract::Query, http::StatusCode, routing::get, Extension, Json, Router};
use data::{db::Database, types::SqlCourse};
use log::info;
use serde::{Deserialize, Serialize};
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};

use crate::data::download_full_database;

mod data;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::try_init()?;
    let db =
        download_full_database("https://blinn-center.github.io/course-scraper/everything.json")
            .await?;
    info!("Loaded {} courses", db.course_count().await?);

    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::any())
        .allow_origin(AllowOrigin::any());

    let app = Router::new()
        .route("/", get(execute_query_request))
        .layer(Extension(Arc::new(db)))
        .layer(cors);

    let bind = "0.0.0.0:3000".parse()?;
    info!("Starting server on {}", bind);
    axum::Server::bind(&bind)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct ExecuteQueryRequestParameters {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
struct ExecuteQueryRequestResults {
    pub query: String,
    pub limit: usize,
    pub result_count: usize,
    pub courses: Vec<SqlCourse>,
}

#[derive(Serialize)]
enum ExecuteQueryRequestMaybeResult {
    #[serde(rename = "data")]
    Success(ExecuteQueryRequestResults),
    #[serde(rename = "error")]
    Error(String),
}

async fn execute_query_request(
    Query(params): Query<ExecuteQueryRequestParameters>,
    Extension(database): Extension<Arc<Database>>,
) -> Result<Json<ExecuteQueryRequestMaybeResult>, (StatusCode, Json<ExecuteQueryRequestMaybeResult>)>
{
    let limit = min(params.limit.unwrap_or(10), 100);
    info!(
        "received query for {:#?} with limit {:#?}",
        &params.q, limit
    );
    let courses = database.search(&params.q, limit).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ExecuteQueryRequestMaybeResult::Error(format!(
                "An error occurred: {}",
                e
            ))),
        )
    })?;
    Ok(Json(ExecuteQueryRequestMaybeResult::Success(
        ExecuteQueryRequestResults {
            query: params.q,
            limit,
            result_count: courses.len(),
            courses,
        },
    )))
}
