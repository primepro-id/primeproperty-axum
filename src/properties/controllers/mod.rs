use crate::db::DbPool;
use axum::routing::{delete, get, post, put};
use axum::Router;

mod configurations;
mod create_update;
mod delete;
mod find;

pub(crate) use configurations::UpdateConfigurationsSqlPayload;
pub(crate) use create_update::CreateUpdatePropertySqlPayload;
pub(crate) use find::{FindPropertyQuery, FindPropertySort, PropertyWithRelation};

pub fn property_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_update::create_property))
        .route("/", get(find::find_many_properties))
        .route("/agents", get(find::find_all_property_agents))
        .route("/agents/{name}", get(find::find_many_by_agent_name))
        .route("/related/{id}", get(find::find_many_related))
        .route("/{id}", get(find::find_one_by_id))
        .route("/{id}", put(create_update::update_property))
        .route("/{id}", delete(delete::delete_property))
        .route(
            "/configurations/{id}",
            put(configurations::update_configurations),
        )
}
