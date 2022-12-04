use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ListResp {
    #[serde(rename = "totalRecords")]
    pub total_records: usize,
    pub records: Vec<CarJson>,
}

#[derive(Serialize, Deserialize)]
pub struct CarJson {
    pub id: i32,
    pub vin: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color: String,
    pub price: BigDecimal,
    #[serde(rename = "modifiedTime")]
    pub updated_at: NaiveDateTime,
}
