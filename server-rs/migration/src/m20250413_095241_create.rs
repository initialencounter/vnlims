use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Project::Project) // 请替换为实际的表名
                    .if_not_exists()
                    .col(integer(Project::Id).auto_increment().primary_key())
                    .col(string(Project::AppraiserName).not_null())
                    .col(string(Project::AssigneeName).not_null())
                    .col(string(Project::AuditorName).null())
                    .col(integer(Project::Conclusions).null())
                    .col(string(Project::DisplayStatus).not_null())
                    .col(boolean(Project::NextYear).null())
                    .col(string(Project::PrincipalName).null())
                    .col(string(Project::ProjectId).not_null())
                    .col(string(Project::ProjectNo).null())
                    .col(boolean(Project::Repeat).not_null())
                    .col(integer(Project::ReportType).not_null())
                    .col(string(Project::SubmitDate).not_null())
                    .col(string(Project::SurveyorNames).null())
                    .col(string(Project::SystemId).not_null())
                    .col(string(Project::SelfId).not_null().unique_key())
                    .col(string(Project::ItemCName).string_len(1024).null())
                    .col(string(Project::ItemEName).string_len(1024).null())
                    .col(string(Project::Mnotes).null())
                    .col(string(Project::ReportNo).string_len(1024).null())
                    .col(string(Project::Tnotes).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Project::Project).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Project {
    #[iden = "project"]
    Project,
    #[iden = "id"]
    Id,
    #[iden = "appraiserName"]
    AppraiserName,
    #[iden = "assigneeName"]
    AssigneeName,
    #[iden = "auditorName"]
    AuditorName,
    #[iden = "conclusions"]
    Conclusions,
    #[iden = "displayStatus"]
    DisplayStatus,
    #[iden = "nextYear"]
    NextYear,
    #[iden = "principalName"]
    PrincipalName,
    #[iden = "projectId"]
    ProjectId,
    #[iden = "projectNo"]
    ProjectNo,
    #[iden = "repeat"]
    Repeat,
    #[iden = "reportType"]
    ReportType,
    #[iden = "submitDate"]
    SubmitDate,
    #[iden = "surveyorNames"]
    SurveyorNames,
    #[iden = "systemId"]
    SystemId,
    #[iden = "selfId"]
    SelfId,
    #[iden = "itemCName"]
    ItemCName,
    #[iden = "itemEName"]
    ItemEName,
    #[iden = "mnotes"]
    Mnotes,
    #[iden = "reportNo"]
    ReportNo,
    #[iden = "tnotes"]
    Tnotes,
}
