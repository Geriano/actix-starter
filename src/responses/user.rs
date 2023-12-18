use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

use crate::models::{permissions, roles, users, Id, Timestamp};

use super::permission::PermissionOAS;
use super::role::RoleOAS;

#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
#[serde(rename_all = "camelCase")]
pub struct UserOAS {
    #[schema()]
    pub id: Id,
    #[schema()]
    pub name: String,
    #[schema()]
    pub email: String,
    #[schema()]
    pub username: String,
    #[schema()]
    pub email_verified_at: Option<Timestamp>,
    #[schema()]
    pub profile_photo_id: Option<String>,
    #[schema()]
    pub permissions: Vec<PermissionOAS>,
    #[schema()]
    pub roles: Vec<RoleOAS>,
}

impl Into<HttpResponse> for UserOAS {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl From<&users::Model> for UserOAS {
    fn from(user: &users::Model) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            email_verified_at: user.email_verified_at,
            profile_photo_id: user.profile_photo_id.clone(),
            permissions: vec![],
            roles: vec![],
        }
    }
}

impl From<(&users::Model, Vec<permissions::Model>, Vec<roles::Model>)> for UserOAS {
    fn from(
        (user, permissions, roles): (&users::Model, Vec<permissions::Model>, Vec<roles::Model>),
    ) -> Self {
        Self {
            id: user.id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            email_verified_at: user.email_verified_at,
            profile_photo_id: user.profile_photo_id.clone(),
            permissions: permissions.iter().map(PermissionOAS::from).collect(),
            roles: roles.iter().map(RoleOAS::from).collect(),
        }
    }
}

impl From<(users::Model, Vec<permissions::Model>, Vec<roles::Model>)> for UserOAS {
    fn from(
        (user, permissions, roles): (users::Model, Vec<permissions::Model>, Vec<roles::Model>),
    ) -> Self {
        Self::from((&user, permissions, roles))
    }
}

#[derive(Serialize, ToSchema, IntoResponses)]
#[serde(rename_all = "camelCase")]
#[response(status = 200, description = "Ok")]
pub struct UserPaginationResponse {
    #[schema(example = "10")]
    pub total: u64,
    #[schema(example = "1")]
    pub page: u64,
    #[schema()]
    pub data: Vec<UserOAS>,
}

impl Into<HttpResponse> for UserPaginationResponse {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
