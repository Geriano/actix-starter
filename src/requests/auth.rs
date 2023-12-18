use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Clone, Deserialize, ToSchema)]
pub struct Login {
    #[schema(example = "john")]
    pub email_or_username: String,
    #[schema(example = "Password123")]
    pub password: String,
}
