use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, QueryResult,
    Queryable, RunQueryDsl,
};
use serde::Serialize;

use crate::db::DbPool;
use crate::schema;

#[derive(Debug, Serialize, Queryable, Clone)]
pub(super) struct Developer {
    id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    picture_url: Option<String>,
    name: String,
    slug: String,
}

impl Developer {
    pub(super) fn find_many(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::developers::table.get_results(conn)
    }
}
