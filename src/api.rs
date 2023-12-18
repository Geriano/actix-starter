use chrono::Local;
use serde_json::json;
use utoipa::openapi::schema::{Schema, SchemaType};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, SchemaFormat};
use utoipa::{Modify, OpenApi, ToSchema};
use uuid::Uuid;

use crate::controllers;
use crate::models;
use crate::requests;
use crate::responses;

#[derive(OpenApi)]
#[openapi(
    modifiers(&Authentication),
    info(
        title = "Learning Management System",
        description = "Learning Management System Service",
        contact(
            name = "Geriano",
            email = "gerznewbie@gmail.com",
            url = "geriano.github.io",
        ),
    ),
    tags(
        (name = "Authentication"),
        (name = "Master User"),
        (name = "Permission"),
        (name = "Role"),
    ),
    paths(
        controllers::auth::login,
        controllers::auth::authenticate,
        controllers::auth::logout,

        controllers::user::paginate,
        controllers::user::store,
        controllers::user::show,
        controllers::user::update_general_information,
        controllers::user::update_password,
        controllers::user::delete,

        controllers::permission::paginate,
        controllers::permission::store,
        controllers::permission::show,
        controllers::permission::update,
        controllers::permission::delete,

        controllers::role::paginate,
        controllers::role::store,
        controllers::role::show,
        controllers::role::update,
        controllers::role::delete,
    ),
    components(
        schemas(T),
        schemas(Id),
        schemas(Timestamp),
        schemas(requests::Sort),
        schemas(requests::PaginationRequest<models::users::Column>),

        schemas(requests::auth::Login),

        schemas(models::users::Column),
        schemas(requests::user::UserStoreRequest),
        schemas(requests::user::UserUpdateGeneralInformationRequest),
        schemas(requests::user::UserUpdatePasswordRequest),

        schemas(models::permissions::Column),
        schemas(requests::permission::PermissionStoreRequest),
        schemas(requests::permission::PermissionUpdateRequest),
        schemas(requests::permission::PermissionBulkRequest),

        schemas(requests::role::RoleStoreRequest),
        schemas(requests::role::RoleUpdateRequest),
        schemas(requests::role::RoleBulkRequest),

        schemas(responses::user::UserOAS),
        schemas(responses::user::UserPaginationResponse),

        schemas(responses::permission::PermissionOAS),
        schemas(responses::permission::PermissionPaginationResponse),

        schemas(responses::role::RoleOAS),
        schemas(responses::role::RolePaginationResponse),
    ),
)]
pub struct Doc;

struct Authentication;

impl Modify for Authentication {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

struct Id;

impl<'__s> ToSchema<'__s> for Id {
    fn schema() -> (&'__s str, RefOr<Schema>) {
        let schema = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Uuid)))
            .example(Some(json!(Uuid::new_v4())))
            .build();

        ("Id", schema.into())
    }
}

struct Timestamp;

impl ToSchema<'_> for Timestamp {
    fn schema() -> (&'static str, RefOr<Schema>) {
        let schema = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
            .example(Some(json!(Local::now())))
            .build();

        ("Timestamp", schema.into())
    }
}

pub struct T;

impl ToSchema<'_> for T {
    fn schema() -> (&'static str, RefOr<Schema>) {
        let schema = ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .enum_values(Some(["name", "createdAt"]))
            .build();

        ("T", schema.into())
    }
}
