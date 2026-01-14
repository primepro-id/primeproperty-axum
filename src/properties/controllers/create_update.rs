use crate::agents::{Agent, AgentRole};
use crate::middleware::Session;
use crate::properties::enumerates::{Currency, RentTime, SoldChannel, SoldStatus};
use crate::properties::model::Property;
use crate::schema;
use crate::traits::Crud;
use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    properties::enumerates::{BuildingCondition, FurnitureCapacity, PurchaseStatus},
};
use axum::extract::Path;
use axum::{
    extract::{Json, State},
    http::HeaderMap,
};
use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Images {
    is_cover: bool,
    path: String,
    english_label: String,
    indonesian_label: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Measurements {
    land_area: Option<i32>,
    building_area: Option<i32>,
    building_level: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Specifications {
    bedrooms: Option<i32>,
    bathrooms: Option<i32>,
    garage: Option<i32>,
    carport: Option<i32>,
    electrical_power: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Facilities {
    value: String,
    indonesian_label: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CreateUpdatePropertyApiPayload {
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: Vec<Images>,
    purchase_status: PurchaseStatus,
    sold_status: Option<SoldStatus>,
    measurements: Measurements,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: Option<String>,
    specifications: Specifications,
    facilities: Vec<Facilities>,
    sold_channel: Option<SoldChannel>,
    currency: Currency,
    rent_time: Option<RentTime>,
    description_seo: Option<String>,
    price_down_payment: Option<i64>,
    developer_id: Option<i32>,
    bank_id: Option<i32>,
}

#[derive(Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = schema::properties)]
pub struct CreateUpdatePropertySqlPayload {
    site_path: String,
    title: String,
    description: String,
    province: String,
    regency: String,
    street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: serde_json::Value,
    purchase_status: PurchaseStatus,
    sold_status: Option<SoldStatus>,
    measurements: serde_json::Value,
    building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: Option<String>,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
    sold_channel: Option<SoldChannel>,
    currency: Currency,
    rent_time: Option<RentTime>,
    description_seo: Option<String>,
    price_down_payment: Option<i64>,
    developer_id: Option<i32>,
    bank_id: Option<i32>,
}

impl CreateUpdatePropertyApiPayload {
    fn to_sql_payload(self) -> CreateUpdatePropertySqlPayload {
        let purchase_status_slug = &self.purchase_status.to_slug();
        let building_type_slug = &self.building_type.trim().replace(" ", "-").to_lowercase();
        let province_slug = &self.province.trim().replace(" ", "-").to_lowercase();
        let regency_slug = &self.regency.trim().replace(" ", "-").to_lowercase();
        let street_slug = &self.street.trim().replace(" ", "-").to_lowercase();
        let site_path =
            format!("/{purchase_status_slug}/{building_type_slug}/{province_slug}/{regency_slug}/{street_slug}");
        CreateUpdatePropertySqlPayload {
            site_path,
            title: self.title.to_string(),
            description: self.description.to_string(),
            province: self.province.trim().to_lowercase(),
            regency: self.regency.trim().to_lowercase(),
            street: self.street.trim().to_lowercase(),
            gmap_iframe: self.gmap_iframe,
            price: self.price,
            images: serde_json::json!(&self.images),
            purchase_status: self.purchase_status,
            sold_status: self.sold_status,
            measurements: serde_json::json!(&self.measurements),
            building_type: self.building_type.to_lowercase(),
            building_condition: self.building_condition,
            building_furniture_capacity: self.building_furniture_capacity,
            building_certificate: match self.building_certificate {
                Some(cert) => Some(cert.to_lowercase()),
                None => None,
            },
            specifications: serde_json::json!(&self.specifications),
            facilities: serde_json::json!(&self.facilities),
            sold_channel: self.sold_channel,
            currency: self.currency,
            rent_time: self.rent_time,
            description_seo: self.description_seo,
            price_down_payment: self.price_down_payment,
            developer_id: self.developer_id,
            bank_id: self.bank_id,
        }
    }
}

pub async fn create_property(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Json(payload): Json<CreateUpdatePropertyApiPayload>,
) -> AxumResponse<Property> {
    let user_id = Session::extract_session_user_id(&headers);
    let sql_payload = payload.to_sql_payload();

    match Property::create(&pool, &user_id, &sql_payload) {
        Ok(property) => JsonResponse::send(201, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}

pub async fn update_property(
    State(pool): State<DbPool>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(payload): Json<CreateUpdatePropertyApiPayload>,
) -> AxumResponse<Property> {
    let user_id = Session::extract_session_user_id(&headers);

    let is_admin = match Agent::find_by_user_id(&pool, &user_id) {
        Ok(agent) => match agent.role {
            AgentRole::Admin => true,
            _ => false,
        },
        Err(err) => return JsonResponse::send(400, None, Some(err.to_string())),
    };

    let property = match Property::find_one_by_id(&pool, &id) {
        Ok(property) => property,
        Err(err) => return JsonResponse::send(400, None, Some(err.to_string())),
    };

    if property.0.user_id != user_id && !is_admin {
        return JsonResponse::send(403, None, Some("Forbidden".to_string()));
    }

    let sql_payload = payload.to_sql_payload();

    match Property::update(&pool, &id, &sql_payload) {
        Ok(property) => JsonResponse::send(200, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
