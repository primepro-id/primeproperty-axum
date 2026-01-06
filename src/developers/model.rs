use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::db::DbPool;
use crate::developers::controller::UpdateDeveloperSqlPayload;
use crate::schema;

#[derive(Debug, Serialize, Queryable, Clone)]
pub(super) struct Developer {
    id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    picture_url: String,
    name: String,
    slug: String,
}

impl Developer {
    pub(super) fn find_many(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::developers::table.get_results(conn)
    }

    pub(super) fn find_by_slug(pool: &DbPool, slug: &str) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        schema::developers::table
            .filter(schema::developers::slug.eq(slug))
            .get_result(conn)
    }

    pub(super) fn create(
        pool: &DbPool,
        picture_url: &str,
        name: &str,
        slug: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");
        diesel::insert_into(schema::developers::table)
            .values((
                schema::developers::picture_url.eq(picture_url),
                schema::developers::name.eq(name),
                schema::developers::slug.eq(slug),
            ))
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        id: &i32,
        payload: &UpdateDeveloperSqlPayload,
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
