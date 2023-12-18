use actix_web::web::{Data, Json, Path, Query};
use actix_web::Responder;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::middlewares::auth::Auth;
use crate::models::users;
use crate::requests::user::{
    UserStoreRequest, UserUpdateGeneralInformationRequest, UserUpdatePasswordRequest,
};
use crate::requests::PaginationRequest;
use crate::responses::user::{UserOAS, UserPaginationResponse};
use crate::responses::{
    CreatedWithId, InternalServerError, NotFound, Ok, Unauthorized, UnprocessableEntity,
};
use crate::services;

/// user pagination
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    params(PaginationRequest),
    responses(
        UserPaginationResponse,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/api/v1/user")]
pub async fn paginate(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Query<PaginationRequest<users::Column>>,
) -> impl Responder {
    services::user::paginate(&db, request.into_inner()).await
}

/// store new user
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    responses(
        CreatedWithId,
        Unauthorized,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[post("/api/v1/user")]
pub async fn store(
    _: Auth,
    db: Data<DatabaseConnection>,
    request: Json<UserStoreRequest>,
) -> impl Responder {
    services::user::store(&db, request.into_inner()).await
}

/// show user by id
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    responses(
        UserOAS,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[get("/api/v1/user/{id}")]
pub async fn show(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::user::show(&db, id.into_inner()).await
}

/// update user by id
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    responses(
        UserOAS,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[put("/api/v1/user/{id}")]
pub async fn update_general_information(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Uuid>,
    request: Json<UserUpdateGeneralInformationRequest>,
) -> impl Responder {
    services::user::update_general_information(&db, id.into_inner(), request.into_inner()).await
}

/// update user password by id
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        UnprocessableEntity,
        InternalServerError,
    ),
)]
#[patch("/api/v1/user/{id}")]
pub async fn update_password(
    _: Auth,
    db: Data<DatabaseConnection>,
    id: Path<Uuid>,
    request: Json<UserUpdatePasswordRequest>,
) -> impl Responder {
    services::user::update_password(&db, id.into_inner(), request.into_inner()).await
}

/// delete user by id
#[utoipa::path(
    tag = "Master User",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        NotFound,
        InternalServerError,
    ),
)]
#[delete("/api/v1/user/{id}")]
pub async fn delete(_: Auth, db: Data<DatabaseConnection>, id: Path<Uuid>) -> impl Responder {
    services::user::delete(&db, id.into_inner()).await
}
