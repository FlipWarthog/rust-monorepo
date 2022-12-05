use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ListResp {
    #[serde(rename = "totalRecords")]
    pub total_records: usize,
    pub records: Vec<CarJson>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CarJson {
    pub id: Option<i32>,
    #[validate(length(max = 40))]
    pub vin: String,
    pub make: String,
    pub model: String,
    #[validate(range(min = 1960))]
    pub year: i32,
    pub color: String,
    pub price: BigDecimal,
    #[serde(rename = "modifiedTime")]
    pub updated_at: Option<NaiveDateTime>,
}
