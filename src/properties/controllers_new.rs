use std::collections::{HashMap, HashSet};

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

pub async fn find_site_paths(State(pool): State<DbPool>) -> AxumResponse<Vec<String>> {
    let distinct_properties = match Property::find_distinct_site_paths(&pool) {
        Ok(properties) => properties,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let slugify = |s: &str| s.to_lowercase().replace(' ', "-");
    let statuses = [
        PurchaseStatus::ForSale.to_slug(),
        PurchaseStatus::ForRent.to_slug(),
    ];

    // Base paths: /for-sale, /for-rent
    let mut site_paths = statuses.iter().map(|s| format!("/{s}")).collect::<Vec<_>>();

    // Extract unique slugs using sets
    let mut building_types = HashSet::new();
    let mut provinces = HashSet::new();
    let mut regencies = HashSet::new();
    let mut streets = HashSet::new();

    for property in distinct_properties {
        building_types.insert(slugify(&property.building_type));
        provinces.insert(slugify(&property.province));
        regencies.insert(slugify(&property.regency));
        streets.insert(slugify(&property.street));
    }

    // Generate paths for each status combination
    for status in &statuses {
        // Status + Building Type
        for b_type in &building_types {
            site_paths.push(format!("/{status}/{b_type}"));

            // Status + Building Type + Province
            for province in &provinces {
                site_paths.push(format!("/{status}/{b_type}/{province}"));

                // Status + Building Type + Province + Regency
                for regency in &regencies {
                    site_paths.push(format!("/{status}/{b_type}/{province}/{regency}"));

                    // Status + Building Type + Province + Regency + Street
                    for street in &streets {
                        site_paths
                            .push(format!("/{status}/{b_type}/{province}/{regency}/{street}"));
                    }
                }
            }
        }
    }

    JsonResponse::send(StatusCode::OK, Some(site_paths), None)
}

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/{id}/join-agents", get(find_unique_join_agent))
        .route("/join-agents", get(find_join_agent))
        .route("/site-paths", get(find_site_paths))
    // let session_routes = Router::new()
    //     .route("/", get(find))
    //     .layer(from_fn(RequestMiddleware::check_session));

    // public_routes.merge(session_routes)
}
