// use super::model::Agent;
// use super::AgentRole;
// use crate::middleware::{JsonFindResponse, Session};
// use crate::supertokens;
use super::model::Agent;
use super::AgentRole;
use crate::{
    db::DbPool,
    envs::Envs,
    mail::Mail,
    middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination, RequestMiddleware},
    schema,
    supertokens::{CreateSessionResponse, SuperTokens, UpdateUserResponse, VerifySessionResponse},
};
use axum::{
    extract::{Json, Path, State},
    http::HeaderMap,
    middleware::from_fn,
    routing::{get, post, put},
};
// use axum::http::{HeaderMap, Method};
// use axum::middleware::{from_fn_with_state, Next};
// use axum::response::Response;
// use axum::routing::{delete, get, post, put};
use axum::Router;
use diesel::{prelude::Insertable, query_builder::AsChangeset};
use reqwest::StatusCode;
use serde::Deserialize;
// use diesel::prelude::{AsChangeset, Insertable};
// use reqwest::StatusCode;
// use serde::Deserialize;
// use tower_http::classify::GrpcCode::Ok;

// async fn middleware(
//     State(pool): State<DbPool>,
//     req: Request,
//     next: Next,
// ) -> Result<Response, AxumResponse<String>> {
//     let headers = req.headers();
//     let user_id = Session::extract_session_user_id(&headers);

//     let agent = match Agent::find_by_user_id(&pool, &user_id) {
//         Ok(agent) => agent,
//         Err(err) => {
//             let response = JsonResponse::send(403, None, Some(err.to_string()));
//             return Err(response);
//         }
//     };

//     match agent.role {
//         AgentRole::Admin => Ok(next.run(req).await),
//         AgentRole::Agent if req.method() == &Method::PUT => Ok(next.run(req).await),
//         _ => {
//             let response = JsonResponse::send(403, None, None);
//             Err(response)
//         }
//     }
// }

// async fn find_agent_by_supertokens_user_id(
//     State(pool): State<DbPool>,
//     Path(id): Path<String>,
// ) -> AxumResponse<Agent> {
//     match Agent::find_by_supertokens_user_id(&pool, &id) {
//         Ok(agent) => JsonResponse::send(200, Some(agent), None),
//         Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
//     }
// }

// #[derive(Deserialize, Insertable, Clone)]
// #[diesel(table_name = schema::agents)]
// pub struct CreateAgentPayload {
//     supertokens_user_id: String,
//     fullname: String,
//     email: String,
//     phone_number: String,
//     profile_picture_url: Option<String>,
//     instagram: Option<String>,
//     description: Option<String>,
// }

// async fn create_agent(
//     State(pool): State<DbPool>,
//     headers: HeaderMap,
//     Json(payload): Json<CreateAgentPayload>,
// ) -> AxumResponse<Agent> {
//     let user_id = Session::extract_session_user_id(&headers);

//     match Agent::find_by_email(&pool, &payload.email) {
//         Ok(_) => JsonResponse::send(400, None, Some("Email already exists".to_string())),
//         _ => {
//             let mut new_payload = payload.clone();
//             new_payload.fullname = payload.fullname.to_lowercase();
//             match Agent::create(&pool, &user_id, &new_payload) {
//                 Ok(agent) => JsonResponse::send(201, Some(agent), None),
//                 Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
//             }
//         }
//     }
// }

// pub(super) const PAGE_SIZE: i64 = 15;
// #[derive(Deserialize)]
// pub struct FindAgentQuery {
//     pub name_or_email: Option<String>,
//     pub page: Option<i64>,
// }
// async fn find_agents(
//     State(pool): State<DbPool>,
//     Query(query): Query<FindAgentQuery>,
// ) -> AxumResponse<JsonFindResponse<Vec<Agent>>> {
//     let agents = match Agent::find_many(&pool, &None, &None, &query) {
//         Ok(agents) => agents,
//         Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
//     };

//     let total_agent_count = match Agent::count_find_many_rows(&pool, &None, &None, &query) {
//         Ok(agents_count) => agents_count,
//         Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
//     };

//     JsonResponse::send(
//         200,
//         Some(JsonFindResponse {
//             data: agents,
//             total_pages: (total_agent_count / PAGE_SIZE) + 1,
//             total_data: total_agent_count,
//         }),
//         None,
//     )
// }

// #[derive(Deserialize, AsChangeset, Clone)]
// #[diesel(table_name = schema::agents)]
// pub struct UpdateAgentPayload {
//     profile_picture_url: Option<String>,
//     fullname: Option<String>,
//     phone_number: Option<String>,
//     instagram: Option<String>,
//     description: Option<String>,
// }

// // for agents to update their information themselves
// async fn update_agent(
//     State(pool): State<DbPool>,
//     headers: HeaderMap,
//     Path(id): Path<String>,
//     Json(payload): Json<UpdateAgentPayload>,
// ) -> AxumResponse<Agent> {
//     let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
//     let user_id = Session::extract_session_user_id(&headers);
//     let mut new_payload = payload.clone();
//     if let Some(fullname) = new_payload.fullname {
//         new_payload.fullname = Some(fullname.to_lowercase());
//     }
//     if agent_id == user_id {
//         match Agent::update_agent(&pool, &agent_id, &new_payload) {
//             Ok(agent) => JsonResponse::send(200, Some(agent), None),
//             Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
//         }
//     } else {
//         match Agent::find_by_user_id(&pool, &user_id) {
//             Ok(agent) => match agent.role {
//                 AgentRole::Admin => match Agent::update_agent(&pool, &agent_id, &new_payload) {
//                     Ok(agent) => JsonResponse::send(200, Some(agent), None),
//                     Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
//                 },
//                 _ => JsonResponse::send(403, None, None),
//             },
//             Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
//         }
//     }
// }

// async fn delete_agent(State(pool): State<DbPool>, Path(id): Path<String>) -> AxumResponse<Agent> {
//     let agent_id = uuid::Uuid::parse_str(&id).expect("Invalid agent id");
//     match Agent::delete_agent(&pool, &agent_id) {
//         Ok(agent) => JsonResponse::send(200, Some(agent), None),
//         Err(err) => return JsonResponse::send(500, None, Some(err.to_string())),
//     }
// }
//

#[derive(Deserialize)]
struct SigninPayload {
    email: String,
    password: String,
}

async fn signin(
    State(pool): State<DbPool>,
    Json(payload): Json<SigninPayload>,
) -> AxumResponse<CreateSessionResponse> {
    let tokens = match SuperTokens::signin(&payload.email, &payload.password).await {
        Ok(s) => s,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    if tokens.status != "OK" {
        return JsonResponse::send(
            StatusCode::BAD_REQUEST,
            None,
            Some("Invalid credentials".to_string()),
        );
    }

    let agent = match Agent::find_by_supertokens_user_id(
        &pool,
        &tokens.recipeUserId.clone().unwrap_or_default(),
    ) {
        Ok(a) => a,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    match SuperTokens::create_session(&tokens.recipeUserId.unwrap_or_default(), &agent).await {
        Ok(s) => JsonResponse::send(StatusCode::OK, Some(s), None),
        Err(e) => JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string())),
    }
}

#[derive(Deserialize)]
struct PasswordResetTokenPayload {
    email: String,
}

async fn create_password_reset_token(
    State(pool): State<DbPool>,
    Json(payload): Json<PasswordResetTokenPayload>,
) -> AxumResponse<String> {
    let agent = match Agent::find_by_email(&pool, &payload.email) {
        Ok(a) => a,
        Err(e) => return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    };

    let token = match SuperTokens::create_password_reset_token(
        &agent.supertokens_user_id.unwrap_or_default(),
        &payload.email,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    if token.status != "OK" {
        return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(token.status));
    }

    let frontend_url = Envs::frontend_url();
    let reset_url = format!(
        "{}/auth/reset-password?token={}",
        frontend_url,
        token.token.unwrap_or_default()
    );
    let body =
        format!("Click here to reset your password: <a href=\"{reset_url}\">{reset_url}</a>");
    match Mail::send(&payload.email, &payload.email, "Reset Password", &body) {
        Ok(_) => JsonResponse::send(StatusCode::OK, Some(token.status), None),
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    }
}

#[derive(Deserialize)]
struct PasswordResetPayload {
    token: String,
    password: String,
}

async fn password_reset(
    Json(payload): Json<PasswordResetPayload>,
) -> AxumResponse<UpdateUserResponse> {
    let token = match SuperTokens::consume_password_reset_token(&payload.token).await {
        Ok(t) => t,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    if token.status != "OK" {
        let res = UpdateUserResponse {
            status: token.status,
        };
        return JsonResponse::send(StatusCode::BAD_REQUEST, Some(res), None);
    }

    match SuperTokens::update_user_password(&token.userId.unwrap_or_default(), &payload.password)
        .await
    {
        Ok(u) => JsonResponse::send(StatusCode::OK, Some(u), None),
        Err(e) => JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string())),
    }
}

#[derive(Deserialize)]
struct CreateAgentPayload {
    fullname: String,
    email: String,
    phone_number: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::agents)]
pub struct CreateAgentFromSupertokensPayload {
    supertokens_user_id: String,
    fullname: String,
    email: String,
    phone_number: String,
}

async fn create(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAgentPayload>,
) -> AxumResponse<Agent> {
    let supertokens = match SuperTokens::signup(&payload.email).await {
        Ok(s) => s,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    if supertokens.status != "OK" {
        return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(supertokens.status));
    }

    let payload = CreateAgentFromSupertokensPayload {
        supertokens_user_id: supertokens.clone().recipeUserId.unwrap_or_default(),
        fullname: payload.fullname.to_lowercase(), // must be lowercase
        email: payload.email,
        phone_number: payload.phone_number,
    };

    let agent = match Agent::create_from_supertokens(&pool, &payload) {
        Ok(a) => a,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    let token = match SuperTokens::create_password_reset_token(
        &supertokens.recipeUserId.unwrap_or_default(),
        &payload.email,
    )
    .await
    {
        Ok(t) => t,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    let frontend_url = Envs::frontend_url();
    let reset_url = format!(
        "{}/auth/reset-password?token={}",
        frontend_url,
        token.token.unwrap_or_default()
    );
    let body =
        format!("Click here to reset your password: <a href=\"{reset_url}\">{reset_url}</a>");
    match Mail::send(
        &payload.email,
        &payload.email,
        "Primepro Indonesia Agent Creation",
        &body,
    ) {
        Ok(_) => JsonResponse::send(StatusCode::OK, Some(agent), None),
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct RefreshSessionPayload {
    refresh_token: String,
}

async fn refresh_session(
    Json(payload): Json<RefreshSessionPayload>,
) -> AxumResponse<CreateSessionResponse> {
    match SuperTokens::refresh_session(&payload.refresh_token).await {
        Ok(s) => JsonResponse::send(StatusCode::OK, Some(s), None),
        Err(e) => JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string())),
    }
}

async fn find(
    State(pool): State<DbPool>,
    // Query(query): Query<FindAgentQuery>,
) -> AxumResponse<DataAndPagination<Vec<Agent>>> {
    let agents = match Agent::find(&pool) {
        Ok(agents) => agents,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };
    let agents_count = match Agent::count(&pool) {
        Ok(agents_count) => agents_count,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let pagination = Pagination::new(None, Some(agents_count as u32), agents_count as u32);
    let data = DataAndPagination::new(Some(agents), pagination);

    JsonResponse::send(StatusCode::OK, Some(data), None)
}

async fn find_unique(State(pool): State<DbPool>, Path(id): Path<String>) -> AxumResponse<Agent> {
    let agent_id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(er) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(er.to_string()),
            )
        }
    };
    match Agent::find_unique(&pool, &agent_id) {
        Ok(a) => JsonResponse::send(StatusCode::OK, Some(a), None),
        Err(err) => JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(err.to_string())),
    }
}

async fn find_by_fullname(
    State(pool): State<DbPool>,
    Path(fullname): Path<String>,
) -> AxumResponse<Agent> {
    let clean_fullname = fullname.to_lowercase().replace("-", " ");
    match Agent::find_by_fullname(&pool, &clean_fullname) {
        Ok(a) => JsonResponse::send(StatusCode::OK, Some(a), None),
        Err(err) => JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(err.to_string())),
    }
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

async fn update(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAgentPayload>,
) -> AxumResponse<Agent> {
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

    let target_user_id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(e) => return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    };

    let session_agent = match Agent::find_unique(&pool, &session_user_id) {
        Ok(agent) => agent,
        Err(e) => {
            return JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string()))
        }
    };

    if target_user_id != session_user_id {
        match session_agent.role {
            AgentRole::Admin => (),
            _ => {
                return JsonResponse::send(
                    StatusCode::FORBIDDEN,
                    None,
                    Some("Forbidden".to_string()),
                )
            }
        }
    }

    match Agent::update(&pool, &target_user_id, &payload) {
        Ok(agent) => JsonResponse::send(StatusCode::OK, Some(agent), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/", get(find))
        .route("/{id}", get(find_unique))
        .route("/fullname/{fullname}", get(find_by_fullname))
        .route("/signin", post(signin))
        .route("/password-reset-token", post(create_password_reset_token))
        .route("/password-reset", post(password_reset))
        .route("/session/refresh", post(refresh_session));

    let session_routes = Router::new()
        .route("/{id}", put(update))
        .layer(from_fn(RequestMiddleware::check_session));

    let admin_routes = Router::new()
        .route("/", post(create))
        .layer(from_fn(RequestMiddleware::check_admin));

    public_routes.merge(session_routes).merge(admin_routes)
    // .route("/", post(create_agent))
    // .route("/", get(find_agents))
    // .route("/{id}", delete(delete_agent))
    // .route("/{id}", put(update_agent))
    // .layer(from_fn_with_state(pool.clone(), middleware))
    // .route("/supertokens/{id}", get(find_agent_by_supertokens_user_id))
}
