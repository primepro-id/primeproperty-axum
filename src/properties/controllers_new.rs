use super::property_relation::PropertyJoinAgent;
use super::Property;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination},
    properties::enumerates::{BuildingCondition, PurchaseStatus, RentTime, SoldStatus},
};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use reqwest::StatusCode;
use serde::Deserialize;

async fn find_unique_join_agent(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<PropertyJoinAgent> {
    match Property::find_unique_join_agent(&pool, &id) {
        Ok(p) => JsonResponse::send(StatusCode::OK, Some(p), None),
        Err(e) => JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(e.to_string())),
    }
}

#[derive(Deserialize)]
pub enum FindQuerySort {
    LowestPrice,
    HighestPrice,
}

#[derive(Deserialize, Default)]
pub struct FindQuery {
    pub id: Option<i32>,
    pub agent_id: Option<uuid::Uuid>,
    pub province: Option<String>,
    pub regency: Option<String>,
    pub street: Option<String>,
    pub purchase_status: Option<PurchaseStatus>,
    pub sold_status: Option<SoldStatus>,
    pub building_type: Option<String>,
    pub building_condition: Option<BuildingCondition>,
    pub keyword: Option<String>,
    pub is_popular: Option<bool>,
    pub is_prime: Option<bool>,
    pub is_related: Option<bool>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<FindQuerySort>,
}

async fn find_join_agent(
    State(pool): State<DbPool>,
    Query(query): Query<FindQuery>,
) -> AxumResponse<DataAndPagination<Vec<PropertyJoinAgent>>> {
    let properties = match Property::find(&pool, &query) {
        Ok(p) => p,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };
    let properties_count = match Property::count(&pool, &query) {
        Ok(c) => c,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let page = match &query.page {
        Some(p) => *p as u32,
        None => 1,
    };

    let per_page = match &query.limit {
        Some(l) => *l as u32,
        None => properties_count as u32,
    };

    let pagination = Pagination::new(Some(page), Some(per_page), properties_count as u32);
    let data = DataAndPagination::new(Some(properties), pagination);

    JsonResponse::send(StatusCode::OK, Some(data), None)
}

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/{id}/agents", get(find_unique_join_agent))
        .route("/agents", get(find_join_agent))
    // let session_routes = Router::new()
    //     .route("/", get(find))
    //     .layer(from_fn(RequestMiddleware::check_session));

    // public_routes.merge(session_routes)
}
