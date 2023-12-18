use actix_web::web::{Data, Json, Path, Query};
use actix_web::Responder;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::middlewares::auth::Auth;
use crate::models::roles;
use crate::requests::role::{RoleStoreRequest, RoleUpdateRequest};
use crate::requests::PaginationRequest;
use crate::responses::role::{RoleOAS, RolePaginationResponse};
use crate::responses::{CreatedWithId, InternalServerError, NotFound, Ok, Unauthorized};
use crate::services;

/// Role pagination
#[utoipa::path(
    tag = "Role",
    security(("token" = [])),
    params(PaginationRequest),
    responses(
        RolePaginationResponse,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/api/v1/role")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Query<PaginationRequest<roles::Column>>,
) -> impl Responder {
    services::role::paginate(&db, request.into_inner()).await
}

/// Store new role
#[utoipa::path(
    tag = "Role",
    security(("token" = [])),
    responses(
        CreatedWithId,
        Unauthorized,
        InternalServerError,
    ),
)]
#[post("/api/v1/role")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<RoleStoreRequest>,
) -> impl Responder {
    services::role::store(&db, request.into_inner()).await
}

/// Get role by id
#[utoipa::path(
    tag = "Role",
    security(("token" = [])),
    responses(
        RoleOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/api/v1/role/{id}")]
pub async fn show(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::role::show(&db, id.into_inner()).await
}

/// Update role by id
#[utoipa::path(
    tag = "Role",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[put("/api/v1/role/{id}")]
pub async fn update(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Uuid>,
    request: Json<RoleUpdateRequest>,
) -> impl Responder {
    services::role::update(&db, id.into_inner(), request.into_inner()).await
}

/// Delete role by id
#[utoipa::path(
    tag = "Role",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/api/v1/role/{id}")]
pub async fn delete(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::role::delete(&db, id.into_inner()).await
}
