use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
    Json,
};
use diesel::prelude::{AsChangeset, Insertable};
use reqwest::Method;
use serde::Deserialize;

use super::model::Bank;
use crate::{
    agents::{Agent, AgentRole},
    db::DbPool,
    middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
    schema,
};

pub(super) async fn banks_middleware(
    State(pool): State<DbPool>,
    req: Request,
    next: Next,
) -> Result<Response, AxumResponse<String>> {
    let method = req.method();
    match method {
        &Method::GET => Ok(next.run(req).await),
        _ => {
            let headers = req.headers();
            let user_id = Session::extract_session_user_id(&headers);

            let agent = match Agent::find_by_user_id(&pool, &user_id) {
                Ok(agent) => agent,
                Err(err) => {
                    let response = JsonResponse::send(403, None, Some(err.to_string()));
                    return Err(response);
                }
            };

            match agent.role {
                AgentRole::Admin => Ok(next.run(req).await),
                _ => {
                    let response = JsonResponse::send(403, None, None);
                    Err(response)
                }
            }
        }
    }
}

pub(super) async fn find_many_banks(
    State(pool): State<DbPool>,
) -> AxumResponse<JsonFindResponse<Vec<Bank>>> {
    let banks = match Bank::find_many(&pool) {
        Ok(banks) => banks,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let res = JsonFindResponse {
        data: banks.clone(),
        total_data: banks.len() as i64,
        total_pages: 1,
    };

    JsonResponse::send(200, Some(res), None)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = schema::banks)]
pub(super) struct CreateBankPayload {
    logo_path: String,
    name: String,
}

pub(super) async fn create_bank(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateBankPayload>,
) -> AxumResponse<Bank> {
    match Bank::create(&pool, &payload) {
        Ok(bank) => JsonResponse::send(201, Some(bank), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = schema::banks)]
pub(super) struct UpdateBankPayload {
    logo_path: Option<String>,
    name: Option<String>,
}

pub(super) async fn update_bank(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBankPayload>,
) -> AxumResponse<Bank> {
    match Bank::update(&pool, &id, &payload) {
        Ok(bank) => JsonResponse::send(200, Some(bank), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub(super) async fn delete_bank(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<Bank> {
    match Bank::delete(&pool, &id) {
        Ok(bank) => JsonResponse::send(200, Some(bank), None),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                JsonResponse::send(404, None, Some("Bank not found".to_string()))
            }
            _ => JsonResponse::send(500, None, Some(err.to_string())),
        },
    }
}

pub(super) async fn find_bank_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<Bank> {
    match Bank::find_by_id(&pool, &id) {
        Ok(bank) => JsonResponse::send(200, Some(bank), None),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                JsonResponse::send(404, None, Some("Bank not found".to_string()))
            }
            _ => JsonResponse::send(500, None, Some(err.to_string())),
        },
    }
}
