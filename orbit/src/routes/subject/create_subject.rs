use migrator::sea_orm::prelude::DateTimeWithTimeZone;
use rocket::response::status;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use subject_quark::Result;
use subject_quark::models::Session as Session;
use subject_quark::r#impl::postgres::subjects::subject::AbstractSubject;
use subject_quark::r#impl::postgres::pool::Db;

/// Subject Create Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataCreateSubject {
    /// Subject name
    pub name: String,

    /// Sex
    pub sex: String,

    /// Status
    pub status: String,

    /// Diagnosis Date
    pub diagnosis_date: DateTimeWithTimeZone,

}

/// Create Subject
#[utoipa::path(
    context_path = "/subject",
    request_body = DataCreateSubject,
    responses(
        (status = 201, description = "subject created successfully"),
    ),
)]
#[post("/create", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session, data: Json<DataCreateSubject>) -> Result<status::NoContent> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let _ = AbstractSubject
        ::create_subject(
            db,
            data.name, 
            data.sex,
            data.status,
            data.diagnosis_date,
        ).await?;

    Ok(status::NoContent)
}
