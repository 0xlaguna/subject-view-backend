use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes::account::login::{DataLoginAccount, LoginResponse, self};
use crate::routes::users::create_user::{DataCreateAccount, self};
use crate::routes::subject::create_subject::{DataCreateSubject, self};
use crate::routes::subject::list_subject::{SubjectListData, SubjectItem, self};
use crate::routes::users::me::{UserGetMeData, self};
use crate::routes::subject::delete_subject::{self};

#[derive(OpenApi)]
#[openapi(
    paths(
        login::req,
        create_user::req,
        create_subject::req,
        delete_subject::req,
        list_subject::req,
        me::req,
    ),
    components(
        schemas(DataLoginAccount, LoginResponse),
        schemas(DataCreateAccount),
        schemas(DataCreateSubject),
        schemas(SubjectListData, SubjectItem),
        schemas(UserGetMeData),
    ),
    tags(
        (name = "SubjectView", description = "SubjectView endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "x-session-token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-session-token"))),
        )
    }
}
