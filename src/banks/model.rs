use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::banks::controller::{CreateBankPayload, UpdateBankPayload};
use crate::db::DbPool;
use crate::schema;

#[derive(Debug, Serialize, Queryable, Clone)]
pub(super) struct Bank {
    pub id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    logo_path: String,
    name: String,
}

impl Bank {
    pub(super) fn find_many(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::banks::table
            .order_by(schema::banks::name.asc())
            .get_results(conn)
    }

    pub(super) fn find_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::banks::table
            .filter(schema::banks::id.eq(id))
            .get_result(conn)
    }

    pub(super) fn create(pool: &DbPool, payload: &CreateBankPayload) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(schema::banks::table)
            .values(payload)
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        id: &i32,
        payload: &UpdateBankPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::update(schema::banks::table)
            .filter(schema::banks::id.eq(id))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(schema::banks::table)
            .filter(schema::banks::id.eq(id))
            .get_result(conn)
    }
}
