use ::entity::{project, project::Entity as Project};
use sea_orm::*;

use crate::SearchParamsNotNull;


pub struct Query;

impl Query {
    pub async fn find_project_by_id(db: &DbConn, id: i32) -> Result<Option<project::Model>, DbErr> {
        Project::find_by_id(id).one(db).await
    }

    /// If ok, returns (project models, num pages).
    pub async fn find_projects_in_page(
        db: &DbConn,
        page: u64,
        projects_per_page: u64,
    ) -> Result<(Vec<project::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Project::find()
            .order_by_asc(project::Column::Id)
            .paginate(db, projects_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated projects
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn search(
        db: &DbConn,
        search_params: SearchParamsNotNull,
    ) -> Result<(Vec<project::Model>, u64), DbErr> {
        let paginator = Project::find()
        .filter(project::Column::SystemId.like(search_params.system_id))
        .filter(project::Column::AppraiserName.like(search_params.appraiser_name))
        .filter(project::Column::ItemCName.like(search_params.item_c_name))
        .filter(project::Column::ItemEName.like(search_params.item_e_name))
        .filter(project::Column::PrincipalName.like(search_params.principal_name))
        .filter(project::Column::ProjectNo.like(search_params.project_no))
        .filter(project::Column::Mnotes.like(search_params.mnotes))
        .filter(project::Column::Tnotes.like(search_params.tnotes))
        .order_by_asc(project::Column::SubmitDate)
        .paginate(db, search_params.rows);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated projects
        paginator.fetch_page(search_params.page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn search_by_field(
        db: &DbConn,
        field: project::Column,
        value: String,
        page: u64,
        rows: u64,
    ) -> Result<(Vec<project::Model>, u64), DbErr> {
        let paginator = Project::find()
            .filter(field.contains(value))
            .order_by(project::Column::SubmitDate, Order::Asc)
            .paginate(db, rows);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
