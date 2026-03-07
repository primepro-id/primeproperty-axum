use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, QueryResult,
    Queryable, RunQueryDsl,
};
use serde::Serialize;

use super::agent_role::AgentRole;
use super::controller::PAGE_SIZE;
use super::controller::{CreateAgentPayload, FindAgentQuery, UpdateAgentPayload};
use crate::db::DbPool;
use crate::schema::agents;

#[derive(Debug, Serialize, Queryable)]
pub struct Agent {
    pub id: uuid::Uuid,
    supertokens_user_id: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    fullname: String,
    email: String,
    phone_number: String,
    profile_picture_url: Option<String>,
    pub role: AgentRole,
    instagram: Option<String>,
    description: Option<String>,
}

impl Agent {
    pub(super) fn find_by_supertokens_user_id(
        pool: &DbPool,
        supertokens_user_id: &str,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table
            .filter(agents::supertokens_user_id.eq(supertokens_user_id))
            .get_result(conn)
    }

    pub fn find_by_user_id(pool: &DbPool, id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table.find(id).get_result(conn)
    }

    pub(super) fn find_by_email(pool: &DbPool, email: &str) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table
            .filter(agents::email.eq(email))
            .get_result(conn)
    }

    pub(super) fn update_agent(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        payload: &UpdateAgentPayload,
    ) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::update(agents::table)
            .filter(agents::id.eq(user_id))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete_agent(pool: &DbPool, user_id: &uuid::Uuid) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::delete(agents::table)
            .filter(agents::id.eq(user_id))
            .get_result(conn)
    }

    pub fn find_by_name(pool: &DbPool, name: &str) -> QueryResult<Self> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table
            .filter(agents::fullname.eq(name))
            .get_result(conn)
    }

    pub fn find_all(pool: &DbPool) -> QueryResult<Vec<Self>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        agents::table
            .filter(agents::email.ne("admin@primeproindonesia.com"))
            .get_results(conn)
    }

    pub fn create(
        pool: &DbPool,
        #[allow(unused_variables)] uuid: &uuid::Uuid,
        payload: &CreateAgentPayload,
    ) -> QueryResult<Agent> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        diesel::insert_into(agents::table)
            .values(payload)
            .get_result(conn)
    }

    pub fn find_many(
        pool: &DbPool,
        #[allow(unused_variables)] user_id: &Option<uuid::Uuid>,
        #[allow(unused_variables)] role: &Option<AgentRole>,
        find_queries: &FindAgentQuery,
    ) -> QueryResult<Vec<Agent>> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let mut query = agents::table
            .filter(agents::role.ne(AgentRole::Admin))
            .order_by(agents::created_at.desc())
            .into_boxed();

        match &find_queries.name_or_email {
            Some(name_or_email) => {
                query = query.filter(
                    agents::fullname
                        .ilike(format!("%{}", name_or_email))
                        .or(agents::fullname.ilike(format!("%{}%", name_or_email)))
                        .or(agents::fullname.ilike(format!("{}%", name_or_email)))
                        .or(agents::email.ilike(format!("%{}", name_or_email)))
                        .or(agents::email.ilike(format!("%{}%", name_or_email)))
                        .or(agents::email.ilike(format!("{}%", name_or_email))),
                );
            }
            None => {}
        }

        match &find_queries.page {
            Some(page) => {
                let offset = (page - 1) * PAGE_SIZE;
                query = query.offset(offset).limit(PAGE_SIZE);
            }
            None => {
                query = query.limit(PAGE_SIZE);
            }
        };

        query.get_results(conn)
    }

    pub fn count_find_many_rows(
        pool: &DbPool,
        #[allow(unused_variables)] user_id: &Option<uuid::Uuid>,
        #[allow(unused_variables)] role: &Option<AgentRole>,
        find_queries: &FindAgentQuery,
    ) -> QueryResult<i64> {
        let conn = &mut pool.get().expect("Couldn't get db connection from pool");

        let mut query = agents::table
            .count()
            .filter(agents::role.ne(AgentRole::Admin))
            .into_boxed();

        match &find_queries.name_or_email {
            Some(name_or_email) => {
                query = query.filter(
                    agents::fullname
                        .ilike(format!("%{}", name_or_email))
                        .or(agents::fullname.ilike(format!("%{}%", name_or_email)))
                        .or(agents::fullname.ilike(format!("{}%", name_or_email)))
                        .or(agents::email.ilike(format!("%{}", name_or_email)))
                        .or(agents::email.ilike(format!("%{}%", name_or_email)))
                        .or(agents::email.ilike(format!("{}%", name_or_email))),
                );
            }
            None => {}
        }

        query.get_result(conn)
    }
}
