use reqwest::Client;
use reqwest::header;
use std::collections::HashMap;
use urlencoding::encode;

use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Spider {
    pub client: Client,
    pub base_url: String,
    pub username: String,
    pub password: String,
}

impl Spider {
    pub fn new(base_url: String, username: String, password: String) -> Self {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        Spider {
            client,
            base_url,
            username,
            password,
        }
    }
    pub async fn login(&self) -> Result<()> {
        let response = self
            .client
            .post(format!("{}/login", &self.base_url))
            .header(
                "Host",
                &self.base_url.replace("http://", "").replace("https://", ""),
            )
            .header("Referer", &self.base_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!(
                "type=password&username={}&password={}",
                &self.username, &self.password
            ))
            .send()
            .await?;

        if response.status().is_success() {
            println!("登录成功");
            Ok(())
        } else {
            Err("登录失败".into())
        }
    }

    pub async fn make_query(&self, query: &str) -> Result<Vec<ProjectModel>> {
        let response = self
            .client
            .get(format!("{}/rest/project?{}", &self.base_url, query))
            .header(
                "Host",
                &self.base_url.replace("http://", "").replace("https://", ""),
            )
            .header("Referer", &self.base_url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await?;

        if response.status().is_success() {
            match response.json::<QueryResponse>().await {
                Ok(query_response) => {
                    return Ok(query_response.rows);
                }
                Err(e) => {
                    println!("解析查询结果失败: {}", e);
                    return Err("解析查询结果失败".into());
                }
            }
        } else {
            Err("查询失败".into())
        }
    }
}

pub fn make_query_string(date: &str, system_id: &str) -> String {
    // 生成 project_no
    let project_no = format!("{}GZ{}", system_id.to_uppercase(), date.replace("-", ""));

    // 创建查询参数
    let mut query_params = HashMap::new();
    query_params.insert("systemId", system_id);
    query_params.insert("category", "battery");
    query_params.insert("reportType", "0");
    query_params.insert("appraiserName", "");
    query_params.insert("itemName", "");
    query_params.insert("principal", "");
    query_params.insert("startDate", date);
    query_params.insert("endDate", date);
    query_params.insert("projectNo", &project_no);
    query_params.insert("page", "1");
    query_params.insert("rows", "10000");

    // 构建查询字符串
    query_params
        .iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_spider() {
        dotenvy::dotenv().ok();

        let lims_base_url =
            env::var("LIMS_BASE_URL").expect("LIMS_BASE_URL is not set in .env file");
        let username = env::var("LIMS_USER_NAME").expect("USER_NAME is not set in .env file");
        let password = env::var("LIMS_PASS_WORD").expect("LIMS_PASS_WORD is not set in .env file");

        println!("base_url: {}", lims_base_url);
        println!("username: {}", username);
        println!("password: {}", password);

        let spider = Spider::new(lims_base_url, username, password);
        spider.login().await.unwrap();

        std::thread::sleep(std::time::Duration::from_secs(2));

        let query = make_query_string("2025-04-11", "sek");
        println!("query: {}\n", query);
        let result = spider.make_query(&query).await.unwrap();
        println!("result: {:?}", result);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct QueryResponse {
    total: i32,
    rows: Vec<ProjectModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectModel {
    pub appraiser_name: String,

    pub assignee_name: String,

    pub auditor_name: Option<String>,

    pub conclusions: i64,

    pub display_status: String,

    pub id: String,

    pub item_c_name: String,

    pub item_e_name: String,

    pub mnotes: String,

    pub next_year: bool,

    pub principal_name: String,

    pub project_id: String,

    pub project_no: String,

    pub repeat: bool,

    pub report_no: Option<String>,

    pub report_type: i64,

    pub submit_date: String,

    pub surveyor_names: Option<String>,

    pub system_id: String,

    pub tnotes: Option<String>,
}
