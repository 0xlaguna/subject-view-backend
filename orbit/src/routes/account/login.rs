use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};

use subject_quark::Result;

use subject_quark::r#impl::postgres::users::account::AbstractAccount;
use subject_quark::models::Session;

use subject_quark::r#impl::postgres::pool::Db;

use utoipa::ToSchema;

/// # Account Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataLoginAccount {
    /// Valid email address
    pub email: String,

    /// Password
    pub password: String,

    /// Session name
    pub name: Option<String>,

}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,

    pub name: String,

    pub user_id: i32,
}

impl From<Session> for LoginResponse {
    fn from(session: Session) -> Self {
        LoginResponse {
            token: session.token,
            name: session.name,
            user_id: session.user_id,
        }
    }
}

/// Login user account
#[utoipa::path(
    context_path = "/account",
    request_body = DataLoginAccount,
    responses(
        (status = 200, description = "Logged in successfully", body = LoginResponse),
    ),
    security(
        (),
        ("x-session-token" = [])
    )
)]
#[post("/login", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, data: Json<DataLoginAccount>) -> Result<Json<LoginResponse>> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let session = AbstractAccount
        ::login(
            db, 
            data.email, 
            data.password, 
            data.name
        ).await?;

    let response: LoginResponse = session.into();
    
    Ok(Json(response))
}