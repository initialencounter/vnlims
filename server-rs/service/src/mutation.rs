use ::entity::project::{self, Column, Entity as Project};
use sea_orm::*;
use spider::ProjectModel;

pub struct Mutation;

impl Mutation {
    pub async fn create_project(
        db: &DbConn,
        form_data: project::Model,
    ) -> Result<project::ActiveModel, DbErr> {
        project::ActiveModel {
            project_no: Set(form_data.project_no.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_project_by_id(
        db: &DbConn,
        id: i32,
        form_data: project::Model,
    ) -> Result<project::Model, DbErr> {
        let project: project::ActiveModel = Project::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find project.".to_owned()))
            .map(Into::into)?;

        project::ActiveModel {
            id: project.id,
            appraiser_name: Set(form_data.appraiser_name.to_owned()),
            assignee_name: Set(form_data.assignee_name.to_owned()),
            auditor_name: Set(form_data.auditor_name.to_owned()),
            conclusions: Set(form_data.conclusions.to_owned()),
            display_status: Set(form_data.display_status.to_owned()),
            next_year: Set(form_data.next_year.to_owned()),
            principal_name: Set(form_data.principal_name.to_owned()),
            project_id: Set(form_data.project_id.to_owned()),
            project_no: Set(form_data.project_no.to_owned()),
            repeat: Set(form_data.repeat.to_owned()),
            submit_date: Set(form_data.submit_date.to_owned()),
            report_type: Set(form_data.report_type.to_owned()),
            surveyor_names: Set(form_data.surveyor_names.to_owned()),
            system_id: Set(form_data.system_id.to_owned()),
            self_id: Set(form_data.self_id.to_owned()),
            item_c_name: Set(form_data.item_c_name.to_owned()),
            item_e_name: Set(form_data.item_e_name.to_owned()),
            mnotes: Set(form_data.mnotes.to_owned()),
            report_no: Set(form_data.report_no.to_owned()),
            tnotes: Set(form_data.tnotes.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn insert_projects(
        db: &DbConn,
        form_data: Vec<ProjectModel>,
    ) -> Result<String, DbErr> {
        if form_data.is_empty() {
            return Ok("no projects to insert".to_string());
        }
        let result = Project::insert_many(zero_copy_batch_convert(form_data))
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(Column::SelfId)
                    .update_columns([
                        Column::DisplayStatus,
                        Column::ReportNo,
                        Column::ReportType,
                        Column::AppraiserName,
                        Column::AssigneeName,
                        Column::AuditorName,
                        Column::Conclusions,
                        Column::NextYear,
                        Column::PrincipalName,
                        Column::ProjectId,
                        Column::ProjectNo,
                        Column::Repeat,
                        Column::SubmitDate,
                        Column::SurveyorNames,
                        Column::SystemId,
                        Column::ItemCName,
                        Column::ItemEName,
                        Column::Mnotes,
                        Column::Tnotes,
                        ])
                    .to_owned(),
            )
            .exec(db)
            .await;
        match result {
            Ok(insert_result) => {
                println!("insert_result: {:?}", insert_result);
            }
            Err(e) => {
                // 针对 RecordNotInserted 错误的处理
                if let DbErr::RecordNotInserted = e {
                    return Ok("RecordNotInserted".to_string());
                }
                println!("insert error: {:?}", e);
                return Ok("other insert error".to_string());
            }
        };
        Ok("projects inserted".to_string())
    }

    pub async fn delete_project(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let project: project::ActiveModel = Project::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find project.".to_owned()))
            .map(Into::into)?;

        project.delete(db).await
    }

    pub async fn delete_all_projects(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Project::delete_many().exec(db).await
    }
}

fn zero_copy_batch_convert(form_data_slice: Vec<ProjectModel>) -> Vec<project::ActiveModel> {
    form_data_slice
        .into_iter()
        .map(|form_data| project::ActiveModel {
            id: NotSet,
            appraiser_name: Set(form_data.appraiser_name),
            assignee_name: Set(form_data.assignee_name),
            auditor_name: Set(form_data.auditor_name),
            conclusions: Set(Some((form_data.conclusions == 1) as i32)),
            display_status: Set(form_data.display_status),
            next_year: Set(Some(form_data.next_year as i8)),
            principal_name: Set(Some(form_data.principal_name)),
            project_id: Set(form_data.project_id),
            project_no: Set(Some(form_data.project_no)),
            repeat: Set(form_data.repeat as i8),
            submit_date: Set(form_data.submit_date),
            report_type: Set(form_data.report_type as i32),
            surveyor_names: Set(form_data.surveyor_names),
            system_id: Set(form_data.system_id),
            self_id: Set(form_data.id),
            item_c_name: Set(Some(form_data.item_c_name)),
            item_e_name: Set(Some(form_data.item_e_name)),
            mnotes: Set(Some(form_data.mnotes)),
            report_no: Set(form_data.report_no),
            tnotes: Set(form_data.tnotes),
        })
        .collect()
}
