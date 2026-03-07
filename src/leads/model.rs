use super::controller::{CreateLeadPayload, FindLeadQueryParam, PAGE_SIZE};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, QueryResult,
    Queryable, RunQueryDsl,
};
use serde::Serialize;

use crate::{agents::AgentRole, db::DbPool, schema::leads};

#[derive(Serialize, Queryable)]
pub struct Lead {
    id: i32,
    user_id: uuid::Uuid,
    property_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    name: String,
    phone_number: String,
    email: Option<String>,
    is_deleted: bool,
}

impl Lead {
    pub fn delete_by_property_id(pool: &DbPool, property_id: &i32) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(leads::table)
            .filter(leads::property_id.eq(property_id))
            .set(leads::is_deleted.eq(true))
            .get_result(conn)
    }

    pub fn create(
        pool: &DbPool,
        #[allow(unused_variables)] uuid: &uuid::Uuid,
        payload: &CreateLeadPayload,
    ) -> QueryResult<Lead> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(leads::table)
            .values(payload)
            .get_result(conn)
    }

    pub fn find_many(
        pool: &DbPool,
        user_id_option: &Option<uuid::Uuid>,
        role_option: &Option<crate::agents::AgentRole>,
        query_params: &FindLeadQueryParam,
    ) -> QueryResult<Vec<Lead>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let mut lead_query = match (user_id_option, role_option) {
            (Some(user_id), Some(role)) => match role {
                &AgentRole::Admin => leads::table.into_boxed(),
                &AgentRole::Agent => leads::table
                    .filter(leads::user_id.eq(user_id).and(leads::is_deleted.eq(false)))
                    .into_boxed(),
            },
            _ => return Err(diesel::result::Error::NotFound),
        };

        if let Some(search) = &query_params.search {
            lead_query = lead_query.filter(
                leads::name
                    .ilike(format!("%{}", search))
                    .or(leads::name.ilike(format!("%{}%", search)))
                    .or(leads::name.ilike(format!("{}%", search)))
                    .or(leads::phone.ilike(format!("%{}", search)))
                    .or(leads::phone.ilike(format!("%{}%", search)))
                    .or(leads::phone.ilike(format!("{}%", search))),
            )
        }

        if let Some(page) = query_params.page {
            lead_query = lead_query.offset((page - 1) * PAGE_SIZE).limit(PAGE_SIZE);
        }

        lead_query
            .order_by(leads::created_at.desc())
            .get_results(conn)
    }

    pub fn count_find_many_rows(
        pool: &DbPool,
        user_id_option: &Option<uuid::Uuid>,
        role_option: &Option<crate::agents::AgentRole>,
        query_params: &FindLeadQueryParam,
    ) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let mut lead_query = match (user_id_option, role_option) {
            (Some(user_id), Some(role)) => match role {
                &AgentRole::Admin => leads::table.into_boxed(),
                &AgentRole::Agent => leads::table
                    .filter(leads::user_id.eq(user_id).and(leads::is_deleted.eq(false)))
                    .into_boxed(),
            },
            _ => return Err(diesel::result::Error::NotFound),
        };

        if let Some(search) = &query_params.search {
            lead_query = lead_query.filter(
                leads::name
                    .ilike(format!("%{}", search))
                    .or(leads::name.ilike(format!("%{}%", search)))
                    .or(leads::name.ilike(format!("{}%", search)))
                    .or(leads::phone.ilike(format!("%{}", search)))
                    .or(leads::phone.ilike(format!("%{}%", search)))
                    .or(leads::phone.ilike(format!("{}%", search))),
            )
        }

        lead_query.count().get_result(conn)
    }
}
