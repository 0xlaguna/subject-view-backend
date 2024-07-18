use migrator::sea_orm::prelude::DateTimeWithTimeZone;
use migrator::sea_orm::ActiveEnum;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use subject_quark::Result;
use subject_quark::models::Session as Session;
use subject_quark::models::subject::{self, Sex, Status};
use subject_quark::r#impl::postgres::subjects::subject::AbstractSubject;
use subject_quark::r#impl::postgres::pool::Db;


/// Subject List Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SubjectListData {
    /// Subject List
    pub list: Vec<SubjectItem>,

    /// Total pages
    pub pages: u64
}

/// Subject Iitem
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SubjectItem {
    /// Id
    pub id: i32,

    /// Subject name
    pub name: String,

    /// Sex
    pub sex: String,

    /// Diagnosis Date
    pub diagnosis_date: DateTimeWithTimeZone,

    /// End Date
    pub created_at: DateTimeWithTimeZone,

    /// Status
    pub status: String,
}

impl From<subject::Model> for SubjectItem {
    fn from(model: subject::Model) -> Self {
        SubjectItem {
            id: model.id,
            name: model.name,
            sex: Sex::to_value(&model.sex),
            diagnosis_date: model.diagnosis_date,
            created_at: model.created_at,
            status: Status::to_value(&model.status)
        }
    }
}

impl From<(Vec<subject::Model>, u64)> for SubjectListData {
    fn from(tuple: (Vec<subject::Model>, u64)) -> Self {
        SubjectListData {
            list: tuple.0.into_iter().map(SubjectItem::from).collect(),
            pages: tuple.1,
        }
    }
}


/// Fetch Subjects
#[utoipa::path(
    context_path = "/subject",
    responses(
        (status = 200, description = "List returned", body = SubjectListData),
    ),
)]
#[get("/?<page>&<per_page>&<search>&<sort_by>&<order>")]
pub async fn req(
    conn: Connection<'_, Db>, 
    mut _session: Session, 
    page: Option<u64>, 
    per_page: Option<u64>,
    search: Option<String>,
    sort_by: Option<String>,
    order: Option<String>
) -> Result<Json<SubjectListData>> { 
    let db = conn.into_inner();

    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(4);
    
    let subjects = AbstractSubject
        ::subject_pagination(db, page, per_page, search, sort_by, order).await?;

    let response: SubjectListData = subjects.into();

    Ok(Json(response))
}
