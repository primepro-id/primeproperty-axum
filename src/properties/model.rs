use super::{
    controllers_new::{CreatePropertySqlPayload, FindQuery, FindQuerySort},
    enumerates::{
        BuildingCondition, Currency, FurnitureCapacity, PurchaseStatus, RentTime, SoldChannel,
        SoldStatus,
    },
    property_relation::PropertyJoinAgent,
};
use crate::{
    agents::AgentRole,
    db::DbPool,
    schema::{agents, properties},
};
use crate::{db::DbPoolExt, properties::controllers_new::UpdatePropertySqlPayload};
use diesel::{
    ExpressionMethods, PgJsonbExpressionMethods, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct Property {
    pub id: i32,
    pub user_id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    site_path: String,
    title: String,
    description: String,
    pub province: String,
    pub regency: String,
    pub street: String,
    gmap_iframe: Option<String>,
    price: i64,
    images: serde_json::Value,
    pub purchase_status: PurchaseStatus,
    sold_status: SoldStatus,
    measurements: serde_json::Value,
    pub building_type: String,
    building_condition: BuildingCondition,
    building_furniture_capacity: Option<FurnitureCapacity>,
    building_certificate: String,
    specifications: serde_json::Value,
    facilities: serde_json::Value,
    is_deleted: bool,
    sold_channel: Option<SoldChannel>,
    configurations: serde_json::Value,
    currency: Currency,
    rent_time: Option<RentTime>,
    description_seo: Option<String>,
    price_down_payment: Option<i64>,
    developer_id: Option<i32>,
    bank_id: Option<i32>,
}

impl DbPoolExt for Property {}

impl Property {
    pub fn find_unique(pool: &DbPool, id: &i32) -> QueryResult<Property> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        properties::table.find(id).get_result(conn)
    }

    pub fn find_unique_join_agent(pool: &DbPool, id: &i32) -> QueryResult<PropertyJoinAgent> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        properties::table
            .find(id)
            .inner_join(agents::table)
            .select((properties::all_columns, agents::all_columns))
            .get_result(conn)
    }

    pub fn find(pool: &DbPool, query: &FindQuery) -> QueryResult<Vec<PropertyJoinAgent>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        let mut property_query = properties::table
            .filter(properties::is_deleted.eq(false))
            .into_boxed();

        if let Some(id) = &query.id {
            if let Some(is_related) = &query.is_related {
                if is_related.to_owned() {
                    property_query = property_query.filter(properties::id.ne(id))
                }
            } else {
                property_query = property_query.filter(properties::id.eq(id))
            }
        }

        if let Some(agent_id) = &query.agent_id {
            property_query = property_query.filter(properties::user_id.eq(agent_id));
        }
        if let Some(province) = &query.province {
            property_query = property_query.filter(properties::province.eq(province));
        }
        if let Some(regency) = &query.regency {
            property_query = property_query.filter(properties::regency.eq(regency));
        }
        if let Some(street) = &query.street {
            property_query = property_query.filter(properties::street.eq(street));
        }
        if let Some(purchase_status) = &query.purchase_status {
            property_query = property_query.filter(properties::purchase_status.eq(purchase_status));
        }
        if let Some(sold_status) = &query.sold_status {
            property_query = property_query.filter(properties::sold_status.eq(sold_status));
        }
        if let Some(building_type) = &query.building_type {
            property_query = property_query.filter(properties::building_type.eq(building_type));
        }
        if let Some(building_condition) = &query.building_condition {
            property_query =
                property_query.filter(properties::building_condition.eq(building_condition));
        }
        if let Some(keyword) = &query.keyword {
            property_query =
                property_query.filter(similarity(properties::site_path, keyword).gt(0.1));
        }

        if let Some(is_popular) = &query.is_popular {
            let filter_json = serde_json::json!({ "is_popular": is_popular});
            property_query = property_query.filter(properties::configurations.contains(filter_json))
        }

        if let Some(is_prime) = &query.is_prime {
            if is_prime.to_owned() {
                property_query = property_query.filter(properties::developer_id.is_not_null())
            }
        }

        match (&query.limit, &query.page) {
            (Some(limit), Some(page)) => {
                let offset = (page - 1) * limit;
                property_query = property_query.offset(offset).limit(limit.to_owned());
            }
            (Some(limit), None) => {
                property_query = property_query.limit(limit.to_owned());
            }
            _ => {}
        }

        match &query.sort {
            Some(sort) => match sort {
                FindQuerySort::LowestPrice => {
                    property_query = property_query.order_by(properties::price.asc())
                }
                FindQuerySort::HighestPrice => {
                    property_query = property_query.order_by(properties::price.desc())
                }
            },
            None => match &query.keyword {
                Some(keyword) => {
                    property_query = property_query.order_by((
                        properties::site_path,
                        similarity(properties::site_path, keyword).desc(),
                    ))
                }
                None => property_query = property_query.order_by(properties::created_at.desc()),
            },
        }

        property_query
            .inner_join(agents::table)
            .select((properties::all_columns, agents::all_columns))
            .get_results::<PropertyJoinAgent>(conn)
    }

    pub fn count(pool: &DbPool, query: &FindQuery) -> QueryResult<i64> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        let mut property_query = properties::table
            .filter(properties::is_deleted.eq(false))
            .into_boxed();

        if let Some(id) = &query.id {
            if let Some(is_related) = &query.is_related {
                if is_related.to_owned() {
                    property_query = property_query.filter(properties::id.ne(id))
                }
            } else {
                property_query = property_query.filter(properties::id.eq(id))
            }
        }

        if let Some(agent_id) = &query.agent_id {
            property_query = property_query.filter(properties::user_id.eq(agent_id));
        }
        if let Some(province) = &query.province {
            property_query = property_query.filter(properties::province.eq(province));
        }
        if let Some(regency) = &query.regency {
            property_query = property_query.filter(properties::regency.eq(regency));
        }
        if let Some(street) = &query.street {
            property_query = property_query.filter(properties::street.eq(street));
        }
        if let Some(purchase_status) = &query.purchase_status {
            property_query = property_query.filter(properties::purchase_status.eq(purchase_status));
        }
        if let Some(sold_status) = &query.sold_status {
            property_query = property_query.filter(properties::sold_status.eq(sold_status));
        }
        if let Some(building_type) = &query.building_type {
            property_query = property_query.filter(properties::building_type.eq(building_type));
        }
        if let Some(building_condition) = &query.building_condition {
            property_query =
                property_query.filter(properties::building_condition.eq(building_condition));
        }
        if let Some(keyword) = &query.keyword {
            property_query =
                property_query.filter(similarity(properties::site_path, keyword).gt(0.1));
        }

        if let Some(is_popular) = &query.is_popular {
            let filter_json = serde_json::json!({ "is_popular": is_popular});
            property_query = property_query.filter(properties::configurations.contains(filter_json))
        }

        if let Some(is_prime) = &query.is_prime {
            if is_prime.to_owned() {
                property_query = property_query.filter(properties::developer_id.is_not_null())
            }
        }

        property_query.count().get_result(conn)
    }

    pub fn find_distinct_site_paths(pool: &DbPool) -> QueryResult<Vec<Property>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        properties::table
            .distinct_on(properties::site_path)
            .order(properties::site_path.asc())
            .get_results(conn)
    }

    pub fn find_navigation(
        pool: &DbPool,
    ) -> QueryResult<Vec<(String, PurchaseStatus, String, String, String, String)>> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };
        properties::table
            .distinct_on(properties::site_path)
            .select((
                properties::site_path,
                properties::purchase_status,
                properties::building_type,
                properties::province,
                properties::regency,
                properties::street,
            ))
            .order(properties::site_path.asc())
            .get_results(conn)
    }

    pub fn create(pool: &DbPool, payload: &CreatePropertySqlPayload) -> QueryResult<Property> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::insert_into(properties::table)
            .values(payload)
            .get_result(conn)
    }

    pub(super) fn update(
        pool: &DbPool,
        id: &i32,
        payload: &UpdatePropertySqlPayload,
    ) -> QueryResult<Property> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        diesel::update(properties::table.filter(properties::id.eq(id)))
            .set(payload)
            .get_result(conn)
    }

    pub(super) fn delete(pool: &DbPool, id: &i32, role: &AgentRole) -> QueryResult<Self> {
        let conn = &mut match pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(Self::to_diesel_error(e)),
        };

        match role {
            AgentRole::Admin => diesel::delete(properties::table)
                .filter(properties::id.eq(id))
                .get_result(conn),
            AgentRole::Agent => diesel::update(properties::table)
                .filter(properties::id.eq(id))
                .set(properties::is_deleted.eq(true))
                .get_result(conn),
        }
    }
}

diesel::define_sql_function! {
    fn similarity(column: diesel::sql_types::Text, keyword: diesel::sql_types::Text) -> diesel::sql_types::Float
}
