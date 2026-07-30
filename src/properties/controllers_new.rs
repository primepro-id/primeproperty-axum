use super::enumerates::{
    BuildingCondition, Currency, FurnitureCapacity, PurchaseStatus, RentTime, SoldStatus,
};
use super::json_model::{Configurations, Facility, Image, Measurement, Specifications};
use super::property_relation::PropertyJoinAgent;
use super::Property;
use crate::middleware::RequestMiddleware;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, DataAndPagination, JsonResponse, Pagination},
    schema,
};
use axum::http::HeaderMap;
use axum::middleware::from_fn;
use axum::{
    extract::{Json, Path, Query, State},
    routing::{get, post},
    Router,
};
use diesel::prelude::Insertable;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

async fn find_site_paths(State(pool): State<DbPool>) -> AxumResponse<Vec<String>> {
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

#[derive(Serialize)]
pub struct PropertyNavigation {
    site_path: String,
    purchase_status: PurchaseStatus,
    building_type: String,
    province: String,
    regency: String,
    street: String,
}

async fn find_navigation(State(pool): State<DbPool>) -> AxumResponse<Vec<PropertyNavigation>> {
    let properties = match Property::find_navigation(&pool) {
        Ok(p) => p,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            )
        }
    };

    let navigation = properties
        .into_iter()
        .map(|n| PropertyNavigation {
            site_path: n.0,
            purchase_status: n.1,
            building_type: n.2,
            province: n.3,
            regency: n.4,
            street: n.5,
        })
        .collect();
    JsonResponse::send(StatusCode::OK, Some(navigation), None)
}

#[derive(Debug, Deserialize)]
pub struct CreatePropertyPayload {
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: Vec<Image>,
    purchase_status: PurchaseStatus,
    measurements: Measurement,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    specifications: Specifications,
    facilities: Vec<Facility>,
    configurations: Vec<Configurations>,
    currency: Currency,
    rent_time: Option<RentTime>,
    price_down_payment: Option<i64>,
}

impl CreatePropertyPayload {
    pub fn into_sql_payload(self, user_id: uuid::Uuid) -> CreatePropertySqlPayload {
        let default_json_object = serde_json::Value::Object(serde_json::Map::new());
        let default_json_array = serde_json::Value::Array(Vec::new());
        CreatePropertySqlPayload {
            user_id,
            title: self.title,
            description: self.description,
            province: self.province,
            regency: self.regency,
            street: self.street,
            gmap_iframe: self.gmap_iframe,
            price: self.price,
            images: serde_json::to_value(self.images).unwrap_or(default_json_array.clone()),
            purchase_status: self.purchase_status,
            measurements: serde_json::to_value(self.measurements)
                .unwrap_or(default_json_object.clone()),
            building_type: self.building_type,
            building_condition: self.building_condition,
            building_furniture_capacity: self.building_furniture_capacity,
            building_certificate: self.building_certificate,
            specifications: serde_json::to_value(self.specifications)
                .unwrap_or(default_json_object.clone()),
            facilities: serde_json::to_value(self.facilities).unwrap_or(default_json_array.clone()),
            configurations: serde_json::to_value(self.configurations)
                .unwrap_or(default_json_object.clone()),
            currency: self.currency,
            rent_time: self.rent_time,
            price_down_payment: self.price_down_payment,
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = schema::properties)]
pub struct CreatePropertySqlPayload {
    user_id: uuid::Uuid,
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: serde_json::Value,
    purchase_status: PurchaseStatus,
    measurements: serde_json::Value,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
    configurations: serde_json::Value,
    currency: Currency,
    rent_time: Option<RentTime>,
    price_down_payment: Option<i64>,
}

async fn create(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreatePropertyPayload>,
) -> AxumResponse<Property> {
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

    let sql_payload = payload.into_sql_payload(session_user_id);
    match Property::create(&pool, &sql_payload) {
        Ok(property) => JsonResponse::send(StatusCode::CREATED, Some(property), None),
        Err(e) => JsonResponse::send(StatusCode::INTERNAL_SERVER_ERROR, None, Some(e.to_string())),
    }
}

pub fn routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/{id}/join-agents", get(find_unique_join_agent))
        .route("/join-agents", get(find_join_agent))
        .route("/site-paths", get(find_site_paths))
        .route("/navigations", get(find_navigation));

    let session_routes = Router::new()
        .route("/", post(create))
        .layer(from_fn(RequestMiddleware::check_session));

    public_routes.merge(session_routes)
}
