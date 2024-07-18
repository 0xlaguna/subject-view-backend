use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::routes::account::login::{DataLoginAccount, LoginResponse, self};
use crate::routes::users::create_user::{DataCreateAccount, self};


#[derive(OpenApi)]
#[openapi(
    paths(
        login::req,
        create_user::req,
    ),
    components(
        schemas(DataLoginAccount, LoginResponse),
        schemas(DataCreateAccount,),
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
