use crate::models::subject::{
    self,
    Model as SubjectModel,
    ActiveModel as SubjectActiveModel,
    Entity as SubjectEntity,
    Sex, Status
};
use chrono::{FixedOffset, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::*;

use crate::{Result, Error};

pub struct AbstractSubject;

impl AbstractSubject {
    
    pub async fn create_subject(
        db: &DbConn,
        name: String,
        sex: String,
        diagnosis_date: DateTimeWithTimeZone
    ) -> Result<SubjectModel> {

        let now_utc = Utc::now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());

        let sex_value = Sex::try_from_value(&sex)
            .map_err(|e| Error::DatabaseError { 
                operation: "create_subject", 
                with: "sessions",
                info: e.to_string()
            })?;

        let subject = SubjectActiveModel {
            id: NotSet,
            name: Set(name),
            sex: Set(sex_value),
            diagnosis_date: Set(diagnosis_date),
            created_at: Set(now_utc),
            status: Set(Status::InScreening),
            updated_at: NotSet
        };

        let subject = subject
            .insert(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "create_subject", 
                with: "sessions",
                info: e.to_string()
            })?;

        Ok(subject)
    }

    /// Paginate subjects
    pub async fn subject_pagination(db: &DbConn, page: u64, subject_per_page: u64) -> Result<(Vec<SubjectModel>, u64)> { 
        let paginator = SubjectEntity::find()
            .order_by_desc(subject::Column::CreatedAt)
            .paginate(db, subject_per_page);

        let num_pages = paginator
            .num_pages()
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "subject_pagination", 
                with: "sessions",
                info: e.to_string()
            })?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|p| (p, num_pages))
            .map_err(|e| Error::DatabaseError {
                operation: "subject_pagination",
                with: "sessions",
                info: e.to_string(),
            })
    }

}
