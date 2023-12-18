use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

use crate::models::permissions::Model;
use crate::models::Id;

#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = 200, description = "Ok")]
pub struct PermissionOAS {
    #[schema()]
    pub id: Id,
    #[schema(example = "CREATE_USER")]
    pub code: String,
    #[schema(example = "create user")]
    pub name: String,
}

impl Into<HttpResponse> for PermissionOAS {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl From<Model> for PermissionOAS {
    fn from(permission: Model) -> Self {
        Self::from(&permission)
    }
}

impl From<&Model> for PermissionOAS {
    fn from(permission: &Model) -> Self {
        Self {
            id: permission.id.clone(),
            code: permission.code.clone(),
            name: permission.name.clone(),
        }
    }
}

#[derive(Serialize, ToSchema, IntoResponses)]
#[serde(rename_all = "camelCase")]
#[response(status = 200, description = "Ok")]
pub struct PermissionPaginationResponse {
    #[schema(example = "10")]
    pub total: u64,
    #[schema(example = "1")]
    pub page: u64,
    #[schema()]
    pub data: Vec<PermissionOAS>,
}

impl Into<HttpResponse> for PermissionPaginationResponse {
    fn into(self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
