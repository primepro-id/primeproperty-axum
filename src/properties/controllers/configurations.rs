use crate::{
    db::DbPool,
    middleware::{AxumResponse, JsonResponse},
    properties::Property,
    schema,
};
use axum::extract::{Json, Path, State};
use diesel::prelude::AsChangeset;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Configurations {
    is_popular: Option<bool>,
    is_njop_price: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UpdateConfigurationsPayload {
    configurations: Configurations,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = schema::properties)]
pub(crate) struct UpdateConfigurationsSqlPayload {
    configurations: serde_json::Value,
}

impl UpdateConfigurationsPayload {
    pub(crate) fn to_sql_payload(&self) -> UpdateConfigurationsSqlPayload {
        UpdateConfigurationsSqlPayload {
            configurations: serde_json::json!(&self.configurations),
        }
    }
}

pub async fn update_configurations(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateConfigurationsPayload>,
) -> AxumResponse<Property> {
    let sql_payload = &payload.to_sql_payload();

    match Property::update_configurations(&pool, &id, sql_payload) {
        Ok(property) => JsonResponse::send(200, Some(property), None),
        Err(err) => JsonResponse::send(500, None, Some(err.to_string())),
    }
}
