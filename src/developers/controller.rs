use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
    Json,
};
use diesel::prelude::{AsChangeset, Insertable};
use reqwest::Method;
use serde::Deserialize;

use crate::{
    agents::{Agent, AgentRole},
    db::DbPool,
    developers::model::Developer,
    middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
    schema,
};

pub(super) async fn developers_middleware(
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

pub(super) async fn find_many_developers(
    State(pool): State<DbPool>,
) -> AxumResponse<JsonFindResponse<Vec<Developer>>> {
    let developers = match Developer::find_many(&pool) {
        Ok(devs) => devs,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let res = JsonFindResponse {
        data: developers.clone(),
        total_data: developers.len() as i64,
        total_pages: 1,
    };

    JsonResponse::send(200, Some(res), None)
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = schema::developers)]
pub(super) struct CreateDeveloperPayload {
    logo_path: String,
    name: String,
}

pub(super) async fn create_developer(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateDeveloperPayload>,
) -> AxumResponse<Developer> {
    match Developer::create(&pool, &payload) {
        Ok(dev) => JsonResponse::send(201, Some(dev), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = schema::developers)]
pub(super) struct UpdateDeveloperPayload {
    logo_path: Option<String>,
    name: Option<String>,
}

pub(super) async fn update_developer(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDeveloperPayload>,
) -> AxumResponse<Developer> {
    match Developer::update(&pool, &id, &payload) {
        Ok(dev) => JsonResponse::send(200, Some(dev), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub(super) async fn delete_developer(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<Developer> {
    match Developer::delete(&pool, &id) {
        Ok(dev) => JsonResponse::send(200, Some(dev), None),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                JsonResponse::send(404, None, Some("Developer not found".to_string()))
            }
            _ => JsonResponse::send(500, None, Some(err.to_string())),
        },
    }
}

pub(super) async fn find_developer_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<Developer> {
    match Developer::find_by_id(&pool, &id) {
        Ok(dev) => JsonResponse::send(200, Some(dev), None),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                JsonResponse::send(404, None, Some("Developer not found".to_string()))
            }
            _ => JsonResponse::send(500, None, Some(err.to_string())),
        },
    }
}
