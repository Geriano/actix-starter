use serde::Deserialize;
use utoipa::openapi::schema::{Schema, SchemaType};
use utoipa::openapi::{ObjectBuilder, RefOr};
use utoipa::ToSchema;

use crate::models::{users, Id};

impl ToSchema<'_> for users::Column {
    fn schema() -> (&'static str, RefOr<Schema>) {
        let schema = ObjectBuilder::new().schema_type(SchemaType::String).build();

        ("PermissionColumn", schema.into())
    }
}

impl<'de> Deserialize<'de> for users::Column {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "name" => Ok(users::Column::Name),
            "email" => Ok(users::Column::Email),
            "username" => Ok(users::Column::Username),
            "createdAt" => Ok(users::Column::CreatedAt),
            _ => Err(serde::de::Error::custom("invalid permission column")),
        }
    }
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct UserStoreRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@local.id")]
    pub email: String,
    #[schema(example = "john")]
    pub username: String,
    #[schema(example = "password")]
    pub password: String,
    #[schema()]
    pub permissions: Vec<Id>,
    #[schema()]
    pub roles: Vec<Id>,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateGeneralInformationRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@local.id")]
    pub email: String,
    #[schema(example = "john")]
    pub username: String,
    #[schema()]
    pub profile_photo_id: Option<String>,
    #[schema()]
    pub permissions: Vec<Id>,
    #[schema()]
    pub roles: Vec<Id>,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdatePasswordRequest {
    #[schema(example = "Password123")]
    pub current_password: String,
    #[schema(example = "Password123")]
    pub new_password: String,
    #[schema(example = "Password123")]
    pub password_confirmation: String,
}
