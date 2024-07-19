use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use subject_quark::Result;
use subject_quark::models::{user, Session as Session};
use subject_quark::r#impl::postgres::users::user::AbstractUser;
use subject_quark::r#impl::postgres::pool::Db;

/// User getme data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserGetMeData {
    /// Email
    pub email: String,

    /// First name
    pub first_name: String,

    /// Middle name
    pub middle_name: String,

    /// Last Name
    pub last_name: String,
}

impl From<user::Model> for UserGetMeData {
    fn from(model: user::Model) -> Self { 
        UserGetMeData {
            email: model.email,
            first_name: model.first_name,
            middle_name: model.middle_name,
            last_name: model.last_name
        }
    }
}

/// Getme
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, body = UserGetMeData, description = "Getme information"),
    ),
)]
#[get("/me")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session) -> Result<Json<UserGetMeData>> {
    let db = conn.into_inner();

    let user = AbstractUser
        ::fetch_user(
            db,
            _session.user_id
        ).await?;

    let user: UserGetMeData = user.into();

    Ok(Json(user))
}
