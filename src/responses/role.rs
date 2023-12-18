use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

use crate::models::roles::Model;
use crate::models::Id;

#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct RoleOAS {
    #[schema()]
    pub id: Id,
    #[schema(example = "SUPERUSER")]
    pub code: String,
    #[schema(example = "superuser")]
    pub name: String,
}

impl Into<HttpResponse> for RoleOAS {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl From<Model> for RoleOAS {
    fn from(role: Model) -> Self {
        Self::from(&role)
    }
}

impl From<&Model> for RoleOAS {
    fn from(role: &Model) -> Self {
        Self {
            id: role.id.clone(),
            code: role.code.clone(),
            name: role.name.clone(),
        }
    }
}

#[derive(Serialize, ToSchema, IntoResponses)]
#[serde(rename_all = "camelCase")]
#[response(status = 200, description = "Ok")]
pub struct RolePaginationResponse {
    #[schema(example = "10")]
    pub total: u64,
    #[schema(example = "1")]
    pub page: u64,
    #[schema()]
    pub data: Vec<RoleOAS>,
}

impl Into<HttpResponse> for RolePaginationResponse {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
