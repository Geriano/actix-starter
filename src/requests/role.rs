use serde::Deserialize;
use utoipa::openapi::schema::{Schema, SchemaType};
use utoipa::openapi::{ObjectBuilder, RefOr};
use utoipa::ToSchema;

use crate::models::{roles, Id};

impl ToSchema<'_> for roles::Column {
    fn schema() -> (&'static str, RefOr<Schema>) {
        let schema = ObjectBuilder::new().schema_type(SchemaType::String).build();

        ("PermissionColumn", schema.into())
    }
}

impl<'de> Deserialize<'de> for roles::Column {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "code" => Ok(roles::Column::Code),
            "name" => Ok(roles::Column::Name),
            _ => Err(serde::de::Error::custom("invalid permission column")),
        }
    }
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct RoleStoreRequest {
    #[schema(example = "AREA_MANAGER")]
    pub code: String,
    #[schema(example = "area manager")]
    pub name: String,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct RoleUpdateRequest {
    #[schema(example = "area manager")]
    pub name: String,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct RoleBulkRequest {
    #[schema()]
    pub roles: Vec<Id>,
}
