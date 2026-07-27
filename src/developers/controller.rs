use super::model::Developer;
use crate::db::DbPool;
use crate::middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination};
use axum::extract::Path;
use axum::{extract::State, routing::get, Router};
use reqwest::StatusCode;

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

pub fn routes() -> Router<DbPool> {
    axum::Router::new()
        .route("/", get(find))
        .route("/{id}", get(find_unique))
}
