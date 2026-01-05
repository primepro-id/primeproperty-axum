use super::controller::find_many_developers;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::Router;

use crate::db::DbPool;
use crate::developers::controller::developers_middleware;

pub fn developers_routes(pool: DbPool) -> Router<DbPool> {
    axum::Router::new()
        .route("/", get(find_many_developers))
        .layer(from_fn_with_state(pool.clone(), developers_middleware))
}
