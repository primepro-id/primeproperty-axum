use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post, put};
use axum::Router;

use crate::banks::controller::{
    banks_middleware, create_bank, delete_bank, find_bank_by_id, find_many_banks, update_bank,
};
use crate::db::DbPool;

pub fn banks_routes(pool: DbPool) -> Router<DbPool> {
    axum::Router::new()
        .route("/", get(find_many_banks))
        .route("/{id}", get(find_bank_by_id))
        .route("/", post(create_bank))
        .route("/{id}", put(update_bank))
        .route("/{id}", delete(delete_bank))
        .layer(from_fn_with_state(pool.clone(), banks_middleware))
}
