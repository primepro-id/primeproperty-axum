use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Image {
    pub path: String,
    pub is_cover: bool,
    pub indonesian_label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Measurement {
    pub land_area: u32,
    pub building_area: u32,
    pub building_level: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Specifications {
    pub garage: u32,
    pub carport: u32,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub electrical_power: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Configurations {
    pub is_popular: Option<bool>,
    pub is_njop_price: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Facility {
    pub value: String,
    pub indonesian_label: String,
}
