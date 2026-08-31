use super::controller::CreateLeadSqlPayload;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::Serialize;

use crate::{
    agents::AgentRole,
    db::{DbPool, DbPoolExt},
    schema,
};

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

impl DbPoolExt for Lead {}

impl Lead {
    pub(super) fn create(pool: &DbPool, payload: &CreateLeadSqlPayload) -> QueryResult<Lead> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::insert_into(schema::leads::table)
            .values(payload)
            .get_result(conn)
    }

    pub fn find(pool: &DbPool, role: &AgentRole, agent_id: &uuid::Uuid) -> QueryResult<Vec<Lead>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        println!("Agent Role and ID: {:?}, {}", role, agent_id);
        match role {
            AgentRole::Admin => schema::leads::table
                .order_by(schema::leads::created_at.desc())
                .get_results(conn),
            AgentRole::Agent => schema::leads::table
                .filter(schema::leads::user_id.eq(agent_id))
                .order_by(schema::leads::created_at.desc())
                .get_results(conn),
        }
    }

    pub fn count(pool: &DbPool, role: &AgentRole, agent_id: &uuid::Uuid) -> QueryResult<i64> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        match role {
            AgentRole::Admin => schema::leads::table.count().get_result(conn),
            AgentRole::Agent => schema::leads::table
                .filter(schema::leads::user_id.eq(agent_id))
                .count()
                .get_result(conn),
        }
    }
}
