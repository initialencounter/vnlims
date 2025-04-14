use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
    Json,
};
use axum_example_service::{
    sea_orm::DatabaseConnection, Mutation as MutationCore, Query as QueryCore, SearchParams,
    SearchParamsNotNull, SearchSingleFieldParams, UpdateProjectsParams,
};
use entity::project::{self, Model};
use serde::{Deserialize, Serialize};
use spider::{make_query_string, Spider};
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    static ref LIMS_BASE_URL: String = env::var("LIMS_BASE_URL").expect("LIMS_BASE_URL is not set in .env file");
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    kind: String,
    message: String,
}

pub async fn create_project(
    state: State<AppState>,
    form: Form<project::Model>,
) -> Result<Json<FlashData>, (StatusCode, &'static str)> {
    let form = form.0;

    MutationCore::create_project(&state.conn, form)
        .await
        .expect("could not insert project");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post successfully added".to_owned(),
    };

    Ok(Json(data))
}

pub async fn delete_project(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<FlashData>, (StatusCode, &'static str)> {
    MutationCore::delete_project(&state.conn, id)
        .await
        .expect("could not delete project");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post successfully deleted".to_owned(),
    };

    Ok(Json(data))
}

pub async fn static_handler() -> Html<&'static [u8]> {
    // 使用 include_bytes! 将 HTML 文件编译进二进制
    Html(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/static/index.html"
    )))
}

pub async fn search_projects(
    state: State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let rows = params.rows.unwrap_or(100);
    let system_id = params.system_id.unwrap_or("".to_string());
    let appraiser_name = params.appraiser_name.unwrap_or("".to_string());
    let item_c_name = params.item_c_name.unwrap_or("".to_string());
    let item_e_name = params.item_e_name.unwrap_or("".to_string());
    let principal_name = params.principal_name.unwrap_or("".to_string());
    let project_no = params.project_no.unwrap_or("".to_string());
    let mnotes = params.mnotes.unwrap_or("".to_string());
    let tnotes = params.tnotes.unwrap_or("".to_string());
    let search_params_not_null = SearchParamsNotNull {
        page,
        rows,
        system_id,
        appraiser_name,
        item_c_name,
        item_e_name,
        principal_name,
        project_no,
        mnotes,
        tnotes,
    };
    println!("\nsearch_params_not_null: {:?}\n", search_params_not_null);
    let (projects, _num_pages) = QueryCore::search(&state.conn, search_params_not_null)
        .await
        .expect("Cannot find projects in page");

    Ok(Json(projects))
}

pub async fn search_by_field(
    state: State<AppState>,
    params: SearchSingleFieldParams,
    field: project::Column,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let rows = params.rows.unwrap_or(100);
    let search_text = params.search_text.unwrap_or("".to_string());

    let (projects, _) = QueryCore::search_by_field(&state.conn, field, search_text, page, rows)
        .await
        .expect("Cannot find projects in page");

    Ok(Json(projects))
}

pub async fn search_t_notes(
    state: State<AppState>,
    Query(params): Query<SearchSingleFieldParams>,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    search_by_field(state, params, project::Column::Tnotes).await
}

pub async fn search_m_notes(
    state: State<AppState>,
    Query(params): Query<SearchSingleFieldParams>,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    search_by_field(state, params, project::Column::Mnotes).await
}

pub async fn search_item_c_name(
    state: State<AppState>,
    Query(params): Query<SearchSingleFieldParams>,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    search_by_field(state, params, project::Column::ItemCName).await
}


pub async fn import_porjects(
    state: State<AppState>,
    Query(params): Query<UpdateProjectsParams>,
) -> String {
    let username = params.username.unwrap_or("".to_string());
    let password = params.password.unwrap_or("".to_string());
    let date = params.date.unwrap_or("".to_string());
    let spider = Spider::new(
        LIMS_BASE_URL.to_string(),
        username,
        password,
    );
    spider.login().await.unwrap();
    let query_string = make_query_string(&date, "pek");
    let form_data = spider.make_query(&query_string).await.unwrap();
    println!("query_response: {:?}", form_data.clone());
    MutationCore::insert_projects(&state.conn, form_data).await.unwrap();
    "".to_string()
}