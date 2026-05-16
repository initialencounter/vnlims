use base64::Engine;
use base64::engine::general_purpose;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::task;
use tokio::time::{Duration, sleep};
use urlencoding::encode;

use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
use regex::Regex;
use std::sync::atomic::{AtomicBool, Ordering};

pub static LOGIN_STATUS: AtomicBool = AtomicBool::new(false);
pub static LOGIN_USERNAME: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36";

lazy_static! {
    static ref RE_PASSWORD_INCORRECT_COUNT: Regex = Regex::new(r"密码错误(\d+)次").unwrap();
    static ref RE_LOGIN_USERNAME: Regex = Regex::new(r"欢迎\s(.+)！").unwrap();
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptchaResponse {
    pub img: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub code: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub password_incorrect_count: u32,
    pub login_username: String,
}

#[derive(Clone)]
pub struct Spider {
    pub client: Client,
    pub host: Arc<RwLock<String>>,
}

impl Spider {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        let host = base_url
            .replace("http://", "")
            .replace("https://", "")
            .split('/')
            .next()
            .unwrap_or("")
            .to_string();
        Spider {
            client,
            host: Arc::new(RwLock::new(host)),
        }
    }
    pub async fn get_captcha(&self) -> Result<CaptchaResponse> {
        let host = self.host.read().unwrap().clone();
        self.log("INFO", &format!("HOST: {}", &host)).await;
        let response = self
            .client
            .get(format!("https://{}/captcha/captchaImage", &host))
            .header("Host", &host)
            .header("Referer", format!("https://{}/login", &host))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        if response.status().is_success() {
            // 获取图片的字节数据
            let image_bytes = response.bytes().await?;
            // 转换为base64编码
            let base64_image = general_purpose::STANDARD.encode(&image_bytes);
            // 构建data URI (假设是PNG格式，也可能是JPEG)
            let img = format!("data:image/jpeg;base64,{}", base64_image);

            self.log("INFO", "获取验证码成功").await;
            Ok(CaptchaResponse { img })
        } else {
            self.log(
                "ERROR",
                &format!("获取验证码失败: {:?}", response.text().await?),
            )
            .await;
            Err("获取验证码失败".into())
        }
    }

    pub async fn login_helper(
        &self,
        response_status: StatusCode,
        response_text: String,
    ) -> LoginResponse {
        if !response_status.is_success() {
            LOGIN_STATUS.store(false, Ordering::Relaxed);
            return LoginResponse {
                success: false,
                message: format!("登录失败, 状态码:{}", response_status.as_u16()),
                password_incorrect_count: 0,
                login_username: "unknow".to_string(),
            };
        }
        let password_incorrect_count = match_password_incorrect_count(response_text.clone());
        let login_username = match_login_username(response_text.clone());
        if response_text.contains("密码错误") {
            self.log(
                "ERROR",
                &format!("登录失败: 密码错误{}次", password_incorrect_count),
            )
            .await;
            return LoginResponse {
                success: false,
                message: format!("密码错误{}次", password_incorrect_count),
                password_incorrect_count,
                login_username,
            };
        } else if response_text.contains("校验码错误") {
            self.log("ERROR", &format!("登录失败: 校验码错误")).await;
            return LoginResponse {
                success: false,
                message: "校验码错误".to_string(),
                password_incorrect_count,
                login_username,
            };
        } else if response_text.contains("欢迎") {
            self.log(
                "INFO",
                &format!("登录成功, 用户名: {}", login_username.clone()),
            )
            .await;
            task::spawn(async {
                sleep(Duration::from_millis(3600 * 1000 * 24)).await;
                LOGIN_STATUS.store(false, Ordering::Relaxed);
                println!("登录状态已过期，请重新登录。");
            });
            LOGIN_STATUS.store(true, Ordering::Relaxed);
            *LOGIN_USERNAME.write().unwrap() = login_username.clone();
            return LoginResponse {
                success: true,
                message: format!("登录成功, 用户名: {}", login_username.clone()),
                password_incorrect_count,
                login_username,
            };
        } else {
            return LoginResponse {
                success: false,
                message: format!("登录失败, 状态码:{}", response_status.as_u16()),
                password_incorrect_count: 0,
                login_username: "unknow".to_string(),
            };
        }
    }

    pub async fn login_with_captcha(
        &self,
        code: &str,
        username: &str,
        password: &str,
    ) -> Result<LoginResponse> {
        self.log("INFO", &format!("username: {}", username)).await;
        self.log("INFO", &format!("password: {}", password)).await;

        let host = self.host.read().unwrap().clone();

        let response = self
            .client
            .post(format!("https://{}/login", &host))
            .header("Host", &host)
            .header("Referer", format!("https://{}/login", &host))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .body(format!(
                "type=password&username={}&password={}&rememberMe=true&validateCode={}",
                urlencoding::encode(username),
                urlencoding::encode(password),
                urlencoding::encode(code)
            ))
            .send()
            .await?;

        let response_status = response.status();
        let response_text = response.text().await?;
        Ok(self.login_helper(response_status, response_text).await)
    }

    pub async fn log(&self, level: &str, message: &str) {
        match level {
            "INFO" => tracing::info!("{}", message),
            "ERROR" => tracing::error!("{}", message),
            _ => {}
        }
    }

    pub async fn make_query(&self, query: &str) -> Result<Vec<ProjectModel>> {
        let host = self.host.read().unwrap().clone();
        println!("Making query to host: {}", &host);
        let response = self
            .client
            .get(format!("https://{}/rest/project?{}", &host, query))
            .header("Host", &host)
            .header("Referer", format!("https://{}/project/main", &host))
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
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
            println!("查询失败: {:?}", response.text().await?);
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

pub fn match_password_incorrect_count(error_message: String) -> u32 {
    RE_PASSWORD_INCORRECT_COUNT
        .captures(&error_message)
        .and_then(|cap| cap[1].parse::<u32>().ok())
        .unwrap_or(0)
}

pub fn match_login_username(welcome_message: String) -> String {
    RE_LOGIN_USERNAME
        .captures(&welcome_message)
        .and_then(|cap| cap[1].parse::<String>().ok())
        .unwrap_or("unknow".to_string())
}
#[cfg(test)]
mod tests {
    use std::{env, fs};

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

        let spider = Spider::new(lims_base_url);
        let captcha = spider.get_captcha().await.unwrap();
        fs::write(
            "captcha.jpg",
            general_purpose::STANDARD
                .decode(captcha.img.replace("data:image/jpeg;base64,", ""))
                .unwrap(),
        )
        .unwrap();
        println!("captcha img saved to captcha.jpg");

        println!("Please input captcha code:");
        let mut code = String::new();
        std::io::stdin().read_line(&mut code).unwrap();
        let code = code.trim();
        match spider.login_with_captcha(code, &username, &password).await {
            Ok(_) => println!("Login successful"),
            Err(e) => {
                println!("Login failed: {}", e);
                return;
            }
        }
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
