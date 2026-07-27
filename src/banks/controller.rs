// use axum::middleware::from_fn_with_state;
use super::model::Bank;
use axum::{extract::State, routing::get, Router};
use reqwest::StatusCode;
// use super::model::Bank;
// use crate::banks::controller::{
//     banks_middleware, create_bank, delete_bank, find_bank_by_id, find_many_banks, update_bank,
// };
use crate::db::DbPool;
use crate::middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination};
// use crate::{
//     agents::{Agent, AgentRole},
//     db::DbPool,
//     middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
//     schema,
// };

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

// #[derive(Debug, Deserialize, Insertable)]
// #[diesel(table_name = schema::banks)]
// pub(super) struct CreateBankPayload {
//     logo_path: String,
//     name: String,
// }

// pub(super) async fn create_bank(
//     State(pool): State<DbPool>,
//     Json(payload): Json<CreateBankPayload>,
// ) -> AxumResponse<Bank> {
//     match Bank::create(&pool, &payload) {
//         Ok(bank) => JsonResponse::send(201, Some(bank), None),
//         Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
//     }
// }

// #[derive(Debug, Deserialize, AsChangeset)]
// #[diesel(table_name = schema::banks)]
// pub(super) struct UpdateBankPayload {
//     logo_path: Option<String>,
//     name: Option<String>,
// }

// pub(super) async fn update_bank(
//     State(pool): State<DbPool>,
//     Path(id): Path<i32>,
//     Json(payload): Json<UpdateBankPayload>,
// ) -> AxumResponse<Bank> {
//     match Bank::update(&pool, &id, &payload) {
//         Ok(bank) => JsonResponse::send(200, Some(bank), None),
//         Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
//     }
// }

// pub(super) async fn delete_bank(
//     State(pool): State<DbPool>,
//     Path(id): Path<i32>,
// ) -> AxumResponse<Bank> {
//     match Bank::delete(&pool, &id) {
//         Ok(bank) => JsonResponse::send(200, Some(bank), None),
//         Err(err) => match err {
//             diesel::result::Error::NotFound => {
//                 JsonResponse::send(404, None, Some("Bank not found".to_string()))
//             }
//             _ => JsonResponse::send(500, None, Some(err.to_string())),
//         },
//     }
// }

// pub(super) async fn find_bank_by_id(
//     State(pool): State<DbPool>,
//     Path(id): Path<i32>,
// ) -> AxumResponse<Bank> {
//     match Bank::find_by_id(&pool, &id) {
//         Ok(bank) => JsonResponse::send(200, Some(bank), None),
//         Err(err) => match err {
//             diesel::result::Error::NotFound => {
//                 JsonResponse::send(404, None, Some("Bank not found".to_string()))
//             }
//             _ => JsonResponse::send(500, None, Some(err.to_string())),
//         },
//     }
// }

pub fn routes() -> Router<DbPool> {
    axum::Router::new().route("/", get(find))
    // .route("/{id}", get(find_bank_by_id))
    // .route("/", post(create_bank))
    // .route("/{id}", put(update_bank))
    // .route("/{id}", delete(delete_bank))
    // .layer(from_fn_with_state(pool.clone(), banks_middleware))
}
