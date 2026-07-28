use super::property_relation::PropertyJoinAgent;
use super::Property;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
};
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use reqwest::StatusCode;

async fn find_unique_join_agent(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<PropertyJoinAgent> {
    match Property::find_unique_join_agent(&pool, &id) {
        Ok(p) => JsonResponse::send(StatusCode::OK, Some(p), None),
        Err(e) => JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    }
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/{id}/agents", get(find_unique_join_agent))

    // let session_routes = Router::new()
    //     .route("/", get(find))
    //     .layer(from_fn(RequestMiddleware::check_session));

    // public_routes.merge(session_routes)
}
