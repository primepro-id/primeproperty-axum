use super::controller::find_many_developers;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::db::DbPool;
use crate::developers::controller::{
    create_developer, delete_developer, developers_middleware, find_developer_by_id,
    update_developer,
};

pub fn developers_routes(pool: DbPool) -> Router<DbPool> {
    axum::Router::new()
        .route("/", get(find_many_developers))
        .route("/{id}", get(find_developer_by_id))
        .route("/", post(create_developer))
        .route("/{id}", put(update_developer))
        .route("/{id}", delete(delete_developer))
        .layer(from_fn_with_state(pool.clone(), developers_middleware))
}
