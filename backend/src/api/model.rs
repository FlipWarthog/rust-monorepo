use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize)]
pub struct ListResp {
    #[serde(rename = "totalRecords")]
    pub total_records: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequestHolder {
    pub request: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    pub first: Option<i64>,
    pub rows: Option<i64>,
    pub page: Option<i64>,
    pub sort_field: Option<String>,
    pub sort_order: Option<i64>,
    pub multi_sort_meta: Vec<MultiSortMeta>,
    pub filters: Option<Filters>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSortMeta {
    pub field: Option<String>,
    pub order: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub vin: Option<FilterMeta>,
    pub make: Option<FilterMeta>,
    pub model: Option<FilterMeta>,
    pub color: Option<FilterMeta>,
    pub year: Option<FilterMeta>,
    pub modified_time: Option<FilterMeta>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterMeta {
    pub operator: String,
    pub constraints: Vec<Constraint>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub value: String,
    pub match_mode: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_deserialize() {
        let json_str = "{\"first\":0,\"rows\":5,\"sortField\":\"\",\"sortOrder\":0,\"multiSortMeta\":[{\"field\":\"make\",\"order\":1},{\"field\":\"model\",\"order\":1}],\"filters\":{\"vin\":{\"operator\":\"or\",\"constraints\":[{\"value\":\"\",\"matchMode\":\"contains\"}]},\"make\":{\"operator\":\"or\",\"constraints\":[{\"value\":\"Ford\",\"matchMode\":\"contains\"}]},\"model\":{\"operator\":\"and\",\"constraints\":[{\"value\":\"\",\"matchMode\":\"contains\"}]},\"color\":{\"operator\":\"or\",\"constraints\":[{\"value\":\"\",\"matchMode\":\"contains\"}]},\"year\":{\"operator\":\"or\",\"constraints\":[{\"value\":\"\",\"matchMode\":\"gte\"}]},\"modifiedTime\":{\"operator\":\"or\",\"constraints\":[{\"value\":\"\",\"matchMode\":\"dateAfter\"}]}}}";

        let value = serde_json::from_str::<QueryRequest>(json_str).unwrap();

        assert_eq!(value.first, Some(0));
        assert_eq!(value.page, None);
    }
}
