use super::model::Developer;
use crate::db::DbPool;
use crate::middleware::{
    AxumResponse, DataAndPagination, JsonResponse, Pagination, RequestMiddleware,
};
use crate::schema;
use axum::middleware::from_fn;
use axum::routing::post;
use axum::{
    extract::{Json, Path, State},
    routing::get,
    Router,
};
use diesel::prelude::Insertable;
use reqwest::StatusCode;
use serde::Deserialize;

pub(super) async fn find(
    State(pool): State<DbPool>,
) -> AxumResponse<DataAndPagination<Vec<Developer>>> {
    let devs = match Developer::find(&pool) {
        Ok(d) => d,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let devs_count = match Developer::count(&pool) {
        Ok(count) => count,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let pagination = Pagination::new(None, Some(devs_count as u32), devs_count as u32);
    let data = DataAndPagination::new(Some(devs), pagination);

    JsonResponse::send(StatusCode::OK, Some(data), None)
}

pub(super) async fn find_unique(
    State(pool): State<DbPool>,

    Path(id): Path<String>,
) -> AxumResponse<Developer> {
    let num_id = match id.parse::<i32>() {
        Ok(num_id) => num_id,
        Err(e) => return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    };
    match Developer::find_unique(&pool, &num_id) {
        Ok(d) => JsonResponse::send(StatusCode::OK, Some(d), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::developers)]
pub(super) struct CreateDeveloperPayload {
    logo_path: String,
    name: String,
}

pub(super) async fn create(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateDeveloperPayload>,
) -> AxumResponse<Developer> {
    match Developer::create(&pool, &payload) {
        Ok(d) => JsonResponse::send(StatusCode::OK, Some(d), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    let public_routes = axum::Router::new()
        .route("/", get(find))
        .route("/{id}", get(find_unique));

    let admin_routes = Router::new()
        .route("/", post(create))
        .layer(from_fn(RequestMiddleware::check_admin));

    public_routes.merge(admin_routes)
}
