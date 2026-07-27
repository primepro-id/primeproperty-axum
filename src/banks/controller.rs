use super::model::Bank;
use crate::db::DbPool;
use crate::middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination};
use axum::{extract::State, routing::get, Router};
use reqwest::StatusCode;

pub(super) async fn find(State(pool): State<DbPool>) -> AxumResponse<DataAndPagination<Vec<Bank>>> {
    let banks = match Bank::find(&pool) {
        Ok(banks) => banks,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let banks_count = match Bank::count(&pool) {
        Ok(count) => count,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let pagination = Pagination::new(None, Some(banks_count as u32), banks_count as u32);
    let data = DataAndPagination::new(Some(banks), pagination);

    JsonResponse::send(StatusCode::OK, Some(data), None)
}

pub fn routes() -> Router<DbPool> {
    axum::Router::new().route("/", get(find))
}
