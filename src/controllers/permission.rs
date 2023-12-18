use actix_web::web::{Data, Json, Path, Query};
use actix_web::Responder;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::middlewares::auth::Auth;
use crate::models::permissions;
use crate::requests::permission::{PermissionStoreRequest, PermissionUpdateRequest};
use crate::requests::PaginationRequest;
use crate::responses::permission::{PermissionOAS, PermissionPaginationResponse};
use crate::responses::{CreatedWithId, InternalServerError, NotFound, Ok, Unauthorized};
use crate::services;

/// Permission pagination
#[utoipa::path(
    tag = "Permission",
    security(("token" = [])),
    params(PaginationRequest),
    responses(
        PermissionPaginationResponse,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/api/v1/permission")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Query<PaginationRequest<permissions::Column>>,
) -> impl Responder {
    services::permission::paginate(&db, request.into_inner()).await
}

/// Store new permission
#[utoipa::path(
    tag = "Permission",
    security(("token" = [])),
    responses(
        CreatedWithId,
        Unauthorized,
        InternalServerError,
    ),
)]
#[post("/api/v1/permission")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<PermissionStoreRequest>,
) -> impl Responder {
    services::permission::store(&db, request.into_inner()).await
}

/// Get permission by id
#[utoipa::path(
    tag = "Permission",
    security(("token" = [])),
    responses(
        PermissionOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/api/v1/permission/{id}")]
pub async fn show(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::permission::show(&db, id.into_inner()).await
}

/// Update permission by id
#[utoipa::path(
    tag = "Permission",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[put("/api/v1/permission/{id}")]
pub async fn update(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Uuid>,
    request: Json<PermissionUpdateRequest>,
) -> impl Responder {
    services::permission::update(&db, id.into_inner(), request.into_inner()).await
}

/// Delete permission by id
#[utoipa::path(
    tag = "Permission",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/api/v1/permission/{id}")]
pub async fn delete(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::permission::delete(&db, id.into_inner()).await
}
