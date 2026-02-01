use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::db::DbPool;
use crate::developers::controller::{CreateDeveloperPayload, UpdateDeveloperPayload};
use crate::schema;

#[derive(Debug, Serialize, Queryable, Clone)]
pub struct Developer {
    pub id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    logo_path: String,
    name: String,
}

impl Developer {
    pub(super) fn find_many(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::developers::table
            .order_by(schema::developers::name.asc())
            .get_results(conn)
    }

    pub(super) fn find_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::developers::table
            .filter(schema::developers::id.eq(id))
            .get_result(conn)
    }

    pub(super) fn create(pool: &DbPool, payload: &CreateDeveloperPayload) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(schema::developers::table)
            .values(payload)
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        id: &i32,
        payload: &UpdateDeveloperPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::update(schema::developers::table)
            .filter(schema::developers::id.eq(id))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::delete(schema::developers::table)
            .filter(schema::developers::id.eq(id))
            .get_result(conn)
    }
}
