use rocket::response::status;
use sea_orm_rocket::Connection;

use subject_quark::Result;
use subject_quark::models::Session as Session;
use subject_quark::r#impl::postgres::subjects::subject::AbstractSubject;
use subject_quark::r#impl::postgres::pool::Db;

/// Delete Subject
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 201, description = "subject deleted successfully"),
    ),
)]
#[delete("/?<id>")]
pub async fn req(
    conn: Connection<'_, Db>,
    mut _session: Session,
    id: i32
) -> Result<status::NoContent> {
    let db = conn.into_inner();

    AbstractSubject
        ::delete_subject(
            db,
            id
        ).await?;

    Ok(status::NoContent)
}
