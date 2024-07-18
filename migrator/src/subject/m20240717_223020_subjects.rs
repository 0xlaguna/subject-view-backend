use extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::subject::model::{Subject, Sex, SexType, Status, StatusType};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> { 

        manager
            .create_type(
                Type::create()
                    .as_enum(Sex::Enum)
                    .values([Sex::Male, Sex::Female])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Enum)
                    .values([Status::InScreening, Status::Enrolled, Status::Failed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subject::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subject::Name).string())
                    .col(
                        ColumnDef::new(Subject::Sex)
                            .enumeration(Sex::Enum, [Sex::Male, Sex::Female])
                    )
                    .col(ColumnDef::new(Subject::DiagnosisDate).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Subject::Status)
                            .enumeration(Status::Enum, [Status::InScreening, Status::Enrolled, Status::Failed])
                    )
                    .col(
                        ColumnDef::new(Subject::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT now()")
                            .not_null()
                    )
                    .col(ColumnDef::new(Subject::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(
            Type::drop().if_exists().name(SexType).to_owned()
        ).await?;
    
        manager.drop_type(
            Type::drop().if_exists().name(StatusType).to_owned()
        ).await?;

        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await?;
        
        Ok(())
    }
}
