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

    /// Build search query with filters
    fn build_search_query(
        search_params: &SearchParamsNotNull,
        start_date: &str,
        end_date: &str,
    ) -> Select<Project> {
        let start = format!("{} 00:00", start_date);
        let end = format!("{} 23:59", end_date);

        Project::find()
            .filter(project::Column::SystemId.like(format!("%{}%", search_params.system_id)))
            .filter(
                project::Column::AppraiserName.like(format!("%{}%", search_params.appraiser_name)),
            )
            .filter(project::Column::ItemCName.like(format!("%{}%", search_params.item_c_name)))
            .filter(project::Column::ItemEName.like(format!("%{}%", search_params.item_e_name)))
            .filter(
                project::Column::PrincipalName.like(format!("%{}%", search_params.principal_name)),
            )
            .filter(project::Column::ProjectNo.like(format!("%{}%", search_params.project_no)))
            .filter(project::Column::Mnotes.like(format!("%{}%", search_params.mnotes)))
            .filter(project::Column::Tnotes.like(format!("%{}%", search_params.tnotes)))
            .filter(
                project::Column::AssigneeName.like(format!("%{}%", search_params.assignee_name)),
            )
            .filter(
                Condition::all()
                    .add(project::Column::SubmitDate.gte(start))
                    .add(project::Column::SubmitDate.lte(end)),
            )
    }

    pub async fn search(
        db: &DbConn,
        search_params: SearchParamsNotNull,
        start_date: &str,
        end_date: &str,
    ) -> Result<(Vec<project::Model>, u64), DbErr> {
        let paginator = Self::build_search_query(&search_params, start_date, end_date)
            .order_by(project::Column::SubmitDate, Order::Desc)
            .paginate(db, search_params.rows);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated projects
        paginator
            .fetch_page(search_params.page - 1)
            .await
            .map(|p| (p, num_pages))
    }

    pub async fn search_count(
        db: &DbConn,
        search_params: SearchParamsNotNull,
        start_date: &str,
        end_date: &str,
    ) -> Result<u64, DbErr> {
        Self::build_search_query(&search_params, start_date, end_date)
            .count(db)
            .await
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
            .order_by(project::Column::SubmitDate, Order::Desc)
            .paginate(db, rows);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
