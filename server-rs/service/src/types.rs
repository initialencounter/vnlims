use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub page: Option<u64>,
    pub rows: Option<u64>,
    pub system_id: Option<String>,
    pub appraiser_name: Option<String>,
    pub item_c_name: Option<String>,
    pub item_e_name: Option<String>,
    pub principal_name: Option<String>,
    pub project_no: Option<String>,
    pub mnotes: Option<String>,
    pub tnotes: Option<String>,
    pub assignee_name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParamsNotNull {
    pub page: u64,
    pub rows: u64,
    pub system_id: String,
    pub appraiser_name: String,
    pub item_c_name: String,
    pub item_e_name: String,
    pub principal_name: String,
    pub project_no: String,
    pub mnotes: String,
    pub tnotes: String,
    pub assignee_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSingleFieldParams {
    pub page: Option<u64>,
    pub rows: Option<u64>,
    pub search_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectsParams {
    pub username: Option<String>,
    pub password: Option<String>,
    pub date: Option<String>,
}
