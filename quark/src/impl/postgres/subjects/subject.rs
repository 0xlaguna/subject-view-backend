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
use sea_query::extension::postgres::PgExpr;

use crate::{Result, Error};

pub struct AbstractSubject;

impl AbstractSubject {
    
    pub async fn create_subject(
        db: &DbConn,
        name: String,
        sex: String,
        status: String,
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

        let status_value = Status::try_from_value(&status)
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
            status: Set(status_value),
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

    /// Delete subject
    pub async fn delete_subject(db: &DbConn, id: i32) -> Result<()> {
        let _res: DeleteResult = SubjectEntity
            ::delete_by_id(id)
            .exec(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "delete_subject", 
                with: "sessions",
                info: e.to_string()
            })?;

        Ok(())
    }

    /// Paginate subjects
    pub async fn subject_pagination(
        db: &DbConn, 
        page: u64, 
        subject_per_page: u64, 
        search: Option<String>,
        sort_by: Option<String>,
        order: Option<String>
    ) -> Result<(Vec<SubjectModel>, u64)> { 
        let paginator = SubjectEntity::find()
            .apply_if(search, |query, v| {
                query.filter(Expr::col(subject::Column::Name).ilike(format!("{}%", v)))
            })
            .apply_if(sort_by, |query, v | {
                let order_value = order.unwrap_or_else(|| "desc".to_owned());

                let sort_order = match order_value.as_str() {
                    "asc" => Order::Asc,
                    _ => Order::Desc,
                };

                let column = match v.as_str() {
                    "name" => subject::Column::Name,
                    "sex" => subject::Column::Sex,
                    "diagnosis_date" => subject::Column::DiagnosisDate,
                    "status" => subject::Column::Status,
                    "created_at" => subject::Column::CreatedAt,
                    _ => subject::Column::Id,
                };

                query.order_by(column, sort_order)
            })
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
