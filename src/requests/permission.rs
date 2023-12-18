use serde::Deserialize;
use utoipa::openapi::schema::{Schema, SchemaType};
use utoipa::openapi::{ObjectBuilder, RefOr};
use utoipa::ToSchema;

use crate::models::{permissions, Id};

impl ToSchema<'_> for permissions::Column {
    fn schema() -> (&'static str, RefOr<Schema>) {
        let schema = ObjectBuilder::new().schema_type(SchemaType::String).build();

        ("PermissionColumn", schema.into())
    }
}

impl<'de> Deserialize<'de> for permissions::Column {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "code" => Ok(permissions::Column::Code),
            "name" => Ok(permissions::Column::Name),
            _ => Err(serde::de::Error::custom("invalid permission column")),
        }
    }
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct PermissionStoreRequest {
    #[schema(example = "CREATE_USER")]
    pub code: String,
    #[schema(example = "create user")]
    pub name: String,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct PermissionUpdateRequest {
    #[schema(example = "create user")]
    pub name: String,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct PermissionBulkRequest {
    #[schema()]
    pub permissions: Vec<Id>,
}
