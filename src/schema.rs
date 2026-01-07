// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "agent_role"))]
    pub struct AgentRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "building_condition"))]
    pub struct BuildingCondition;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "currency_unit"))]
    pub struct CurrencyUnit;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "furniture_capacity"))]
    pub struct FurnitureCapacity;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "purchase_status"))]
    pub struct PurchaseStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rent_time_unit"))]
    pub struct RentTimeUnit;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sold_channel"))]
    pub struct SoldChannel;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "sold_status"))]
    pub struct SoldStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AgentRole;

    agents (id) {
        id -> Uuid,
        supertokens_user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        fullname -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        profile_picture_url -> Nullable<Varchar>,
        role -> AgentRole,
        #[max_length = 255]
        instagram -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    developers (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        logo_path -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    leads (id) {
        id -> Int4,
        user_id -> Uuid,
        property_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PurchaseStatus;
    use super::sql_types::SoldStatus;
    use super::sql_types::BuildingCondition;
    use super::sql_types::FurnitureCapacity;
    use super::sql_types::SoldChannel;
    use super::sql_types::CurrencyUnit;
    use super::sql_types::RentTimeUnit;

    properties (id) {
        id -> Int4,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        site_path -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        province -> Varchar,
        #[max_length = 255]
        regency -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        gmap_iframe -> Nullable<Text>,
        price -> Int8,
        images -> Jsonb,
        purchase_status -> PurchaseStatus,
        sold_status -> SoldStatus,
        measurements -> Jsonb,
        #[max_length = 255]
        building_type -> Varchar,
        building_condition -> BuildingCondition,
        building_furniture_capacity -> Nullable<FurnitureCapacity>,
        #[max_length = 255]
        building_certificate -> Varchar,
        specifications -> Jsonb,
        facilities -> Jsonb,
        is_deleted -> Bool,
        sold_channel -> Nullable<SoldChannel>,
        configurations -> Jsonb,
        currency -> CurrencyUnit,
        rent_time -> Nullable<RentTimeUnit>,
        #[max_length = 255]
        description_seo -> Nullable<Varchar>,
        price_down_payment -> Nullable<Int8>,
        developer_id -> Nullable<Int4>,
    }
}

diesel::joinable!(leads -> agents (user_id));
diesel::joinable!(leads -> properties (property_id));
diesel::joinable!(properties -> agents (user_id));
diesel::joinable!(properties -> developers (developer_id));

diesel::allow_tables_to_appear_in_same_query!(agents, developers, leads, properties,);
