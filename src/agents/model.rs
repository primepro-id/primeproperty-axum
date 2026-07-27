use super::agent_role::AgentRole;
use crate::agents::controller::{CreateAgentFromSupertokensPayload, UpdateAgentPayload};
use crate::db::{DbPool, DbPoolExt};
use crate::schema::agents;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Clone, Deserialize)]
pub struct Agent {
    pub id: uuid::Uuid,
    pub supertokens_user_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    fullname: String,
    pub email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
    pub role: AgentRole,
    instagram: Option<String>,
    description: Option<String>,
}

impl DbPoolExt for Agent {}

impl Agent {
    pub(super) fn find(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table
            .filter(agents::role.ne(AgentRole::Admin))
            .get_results(conn)
    }

    pub(super) fn count(pool: &DbPool) -> QueryResult<i64> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table
            .filter(agents::role.ne(AgentRole::Admin))
            .count()
            .get_result(conn)
    }

    pub(super) fn find_unique(pool: &DbPool, id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table.find(id).get_result(conn)
    }

    pub(super) fn find_by_supertokens_user_id(
        pool: &DbPool,
        supertokens_user_id: &str,
    ) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table
            .filter(agents::supertokens_user_id.eq(supertokens_user_id))
            .get_result(conn)
    }

    pub(super) fn find_by_fullname(pool: &DbPool, fullname: &str) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table
            .filter(agents::fullname.eq(fullname))
            .get_result(conn)
    }

    pub(super) fn find_by_email(pool: &DbPool, email: &str) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        agents::table
            .filter(agents::email.eq(email))
            .get_result(conn)
    }

    pub(super) fn create_from_supertokens(
        pool: &DbPool,
        payload: &CreateAgentFromSupertokensPayload,
    ) -> QueryResult<Agent> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::insert_into(agents::table)
            .values(payload)
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &UpdateAgentPayload,
    ) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::update(agents::table)
            .filter(agents::id.eq(user_id))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::delete(agents::table)
            .filter(agents::id.eq(user_id))
            .get_result(conn)
    }
}
