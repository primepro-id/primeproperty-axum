use super::model::Agent;
use super::AgentRole;
use crate::middleware::{JsonFindResponse, Session};
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    schema,
};
use axum::extract::{Json, Path, Query, Request, State};
use axum::http::{HeaderMap, Method};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::Response;
use axum::routing::{delete, get, post, put};
use axum::Router;
use diesel::prelude::{AsChangeset, Insertable};
use serde::Deserialize;

async fn middleware(
    State(pool): State<DbPool>,
    req: Request,
    next: Next,
) -> Result<Response, AxumResponse<String>> {
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
        AgentRole::Agent if req.method() == &Method::PUT => Ok(next.run(req).await),
        _ => {
            let response = JsonResponse::send(403, None, None);
            Err(response)
        }
    }
}

async fn find_agent_by_supertokens_user_id(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> AxumResponse<Agent> {
    match Agent::find_by_supertokens_user_id(&pool, &id) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = schema::agents)]
pub struct CreateAgentPayload {
    supertokens_user_id: String,
    fullname: String,
    email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
    instagram: Option<String>,
    description: Option<String>,
}

async fn create_agent(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateAgentPayload>,
) -> AxumResponse<Agent> {
    let user_id = Session::extract_session_user_id(&headers);

    match Agent::find_by_email(&pool, &payload.email) {
        Ok(_) => JsonResponse::send(400, None, Some("Email already exists".to_string())),
        _ => {
            let mut new_payload = payload.clone();
            new_payload.fullname = payload.fullname.to_lowercase();
            match Agent::create(&pool, &user_id, &new_payload) {
                Ok(agent) => JsonResponse::send(201, Some(agent), None),
                Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
            }
        }
    }
}

pub(super) const PAGE_SIZE: i64 = 15;
#[derive(Deserialize)]
pub struct FindAgentQuery {
    pub name_or_email: Option<String>,
    pub page: Option<i64>,
}
async fn find_agents(
    State(pool): State<DbPool>,
    Query(query): Query<FindAgentQuery>,
) -> AxumResponse<JsonFindResponse<Vec<Agent>>> {
    let agents = match Agent::find_many(&pool, &None, &None, &query) {
        Ok(agents) => agents,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    let total_agent_count = match Agent::count_find_many_rows(&pool, &None, &None, &query) {
        Ok(agents_count) => agents_count,
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    };

    JsonResponse::send(
        200,
        Some(JsonFindResponse {
            data: agents,
            total_pages: (total_agent_count / PAGE_SIZE) + 1,
            total_data: total_agent_count,
        }),
        None,
    )
}

#[derive(Deserialize, AsChangeset, Clone)]
#[diesel(table_name = schema::agents)]
pub struct UpdateAgentPayload {
    profile_picture_url: Option<String>,
    fullname: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    description: Option<String>,
}

// for agents to update their information themselves
async fn update_agent(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAgentPayload>,
) -> AxumResponse<Agent> {
    let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
    let user_id = Session::extract_session_user_id(&headers);
    let mut new_payload = payload.clone();
    if let Some(fullname) = new_payload.fullname {
        new_payload.fullname = Some(fullname.to_lowercase());
    }
    if agent_id == user_id {
        match Agent::update_agent(&pool, &agent_id, &new_payload) {
            Ok(agent) => JsonResponse::send(200, Some(agent), None),
            Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
        }
    } else {
        match Agent::find_by_user_id(&pool, &user_id) {
            Ok(agent) => match agent.role {
                AgentRole::Admin => match Agent::update_agent(&pool, &agent_id, &new_payload) {
                    Ok(agent) => JsonResponse::send(200, Some(agent), None),
                    Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
                },
                _ => JsonResponse::send(403, None, None),
            },
            Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
        }
    }
}

async fn delete_agent(State(pool): State<DbPool>, Path(id): Path<String>) -> AxumResponse<Agent> {
    let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
    match Agent::delete_agent(&pool, &agent_id) {
        Ok(agent) => JsonResponse::send(200, Some(agent), None),
        Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub fn agent_routes(pool: DbPool) -> Router<DbPool> {
    Router::new()
        .route("/", post(create_agent))
        .route("/", get(find_agents))
        .route("/{id}", delete(delete_agent))
        .route("/{id}", put(update_agent))
        .layer(from_fn_with_state(pool.clone(), middleware))
        .route("/supertokens/{id}", get(find_agent_by_supertokens_user_id))
}
