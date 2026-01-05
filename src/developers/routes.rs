use super::controller::find_many_developers;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post, put};
use axum::Router;

use crate::db::DbPool;
use crate::developers::controller::{create_developer, developers_middleware, update_developer};

pub fn developers_routes(pool: DbPool) -> Router<DbPool> {
    axum::Router::new()
        .route("/", get(find_many_developers))
        .route("/", post(create_developer))
        .route("/{id}", put(update_developer))
        .layer(from_fn_with_state(pool.clone(), developers_middleware))
}
