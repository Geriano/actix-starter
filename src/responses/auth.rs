use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

use super::user::UserOAS;

#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct Login {
    #[schema()]
    pub token: String,
    #[schema()]
    pub user: UserOAS,
}

#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct Authenticated {
    #[schema()]
    pub user: UserOAS,
}
