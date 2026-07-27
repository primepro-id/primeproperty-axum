use crate::db::{DbPool, DbPoolExt};
use crate::schema;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, Clone)]
pub(super) struct Bank {
    pub id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    logo_path: String,
    name: String,
}

impl DbPoolExt for Bank {}

impl Bank {
    pub(super) fn find(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };
        schema::banks::table
            .order_by(schema::banks::name.asc())
            .get_results(conn)
    }

    pub(super) fn count(pool: &DbPool) -> QueryResult<i64> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        schema::banks::table.count().get_result(conn)
    }
}
