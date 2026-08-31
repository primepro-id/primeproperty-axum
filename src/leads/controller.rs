use super::model::Lead;
use crate::agents::Agent;
use crate::middleware::{DataAndPagination, JsonResponse, Pagination, RequestMiddleware};
use crate::properties::Property;
use crate::{db::DbPool, middleware::AxumResponse, schema};
use axum::extract::{Json, State};
use axum::http::HeaderMap;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::Router;
use diesel::prelude::Insertable;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateLeadPayload {
    property_id: i32,
    name: String,
    phone: String,
    email: Option<String>,
}

impl CreateLeadPayload {
    fn into_sql_payload(self, user_id: &uuid::Uuid) -> CreateLeadSqlPayload {
        CreateLeadSqlPayload {
            user_id: user_id.to_owned(),
            property_id: self.property_id,
            name: self.name,
            phone: self.phone,
            email: self.email,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::leads)]
pub(super) struct CreateLeadSqlPayload {
    user_id: uuid::Uuid,
    property_id: i32,
    name: String,
    phone: String,
    email: Option<String>,
}

async fn create(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateLeadPayload>,
) -> AxumResponse<Lead> {
    let property = match Property::find_unique(&pool, &payload.property_id) {
        Ok(p) => p,
        Err(e) => return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    };

    let sql_payload = payload.into_sql_payload(&property.user_id);
    match Lead::create(&pool, &sql_payload) {
        Ok(lead) => JsonResponse::send(StatusCode::CREATED, Some(lead), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

async fn find(
    State(pool): State<DbPool>,
    headers: HeaderMap,
) -> AxumResponse<DataAndPagination<Vec<Lead>>> {
    let session_user_id = match RequestMiddleware::get_user_uuid(&headers) {
        Some(id) => id,
        None => {
            return JsonResponse::send(
                StatusCode::UNAUTHORIZED,
                None,
                Some("Unauthorized".to_string()),
            )
        }
    };

    let session_agent = match Agent::find_unique(&pool, &session_user_id) {
        Ok(agent) => agent,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    let leads = match Lead::find(&pool, &session_agent.role, &session_agent.id) {
        Ok(l) => l,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let leads_count = match Lead::count(&pool, &session_agent.role, &session_agent.id) {
        Ok(c) => c,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let pagination = Pagination::new(None, Some(leads_count as u32), leads_count as u32);
    let data = DataAndPagination::new(Some(leads), pagination);

    JsonResponse::send(StatusCode::OK, Some(data), None)
}

pub fn routes() -> Router<DbPool> {
    let public_routes = Router::new().route("/", post(create));

    let session_routes = Router::new()
        .route("/", get(find))
        .layer(from_fn(RequestMiddleware::check_session));

    public_routes.merge(session_routes)
}
