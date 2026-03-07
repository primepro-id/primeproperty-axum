use crate::agents::Agent;
use crate::middleware::{JsonFindResponse, JsonResponse, Session};
use crate::properties::Property;
use crate::{db::DbPool, middleware::AxumResponse, schema};
use axum::extract::{Json, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::Router;
use diesel::prelude::Insertable;
use serde::Deserialize;

use super::model::Lead;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::leads)]
pub struct CreateLeadPayload {
    user_id: uuid::Uuid,
    property_id: i32,
    name: String,
    phone: String,
    email: Option<String>,
}

async fn create_lead(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateLeadPayload>,
) -> AxumResponse<Lead> {
    let api_key_option = headers.get("x-api-key");

    let api_key = match api_key_option {
        Some(key) => key.to_str().unwrap_or(""),
        None => return JsonResponse::send(401, None, None),
    };

    let leads_api_key = std::env::var("API_KEY_LEADS").expect("Missing API_KEY_LEADS");

    if api_key != leads_api_key {
        return JsonResponse::send(401, None, None);
    }

    let property = match Property::find_one_by_id(&pool, &payload.property_id) {
        Ok(property) if property.0.user_id == payload.user_id => property,
        _ => return JsonResponse::send(400, None, None),
    };
    match Lead::create(&pool, &property.0.user_id, &payload) {
        Ok(lead) => JsonResponse::send(201, Some(lead), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub(super) const PAGE_SIZE: i64 = 20;

#[derive(Deserialize)]
pub struct FindLeadQueryParam {
    pub search: Option<String>,
    pub page: Option<i64>,
}

async fn find_many_leads(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Query(query_params): Query<FindLeadQueryParam>,
) -> AxumResponse<JsonFindResponse<Vec<Lead>>> {
    let user_id = Session::extract_session_user_id(&headers);

    let role = match Agent::find_by_user_id(&pool, &user_id) {
        Ok(agent) => Some(agent.role),
        _ => {
            return JsonResponse::send(403, None, None);
        }
    };

    let leads = match Lead::find_many(&pool, &Some(user_id), &role, &query_params) {
        Ok(leads_vec) => leads_vec,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let leads_count = match Lead::count_find_many_rows(&pool, &Some(user_id), &role, &query_params)
    {
        Ok(count) => count,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let body = JsonFindResponse {
        data: leads,
        total_pages: (leads_count / PAGE_SIZE) + 1,
        total_data: leads_count,
    };
    JsonResponse::send(200, Some(body), None)
}

pub fn lead_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_lead))
        .route("/", get(find_many_leads))
}
