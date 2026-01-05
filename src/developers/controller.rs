use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    agents::{Agent, AgentRole},
    db::DbPool,
    developers::model::Developer,
    middleware::{AxumResponse, JsonFindResponse, JsonResponse, Session},
};

pub(super) async fn developers_middleware(
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
        _ => {
            let response = JsonResponse::send(403, None, None);
            Err(response)
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
