use actix_web::web::{Data, Json};
use actix_web::Responder;
use sea_orm::DatabaseConnection;

use crate::middlewares::auth::Auth;
use crate::responses;
use crate::responses::{InternalServerError, Ok, Unauthorized, UnprocessableEntity};
use crate::{requests::auth::Login, services};

/// Login by email or username
#[utoipa::path(
    tag = "Authentication",
    responses(responses::auth::Login, UnprocessableEntity, InternalServerError,)
)]
#[post("/login")]
pub async fn login(db: Data<DatabaseConnection>, request: Json<Login>) -> impl Responder {
    services::auth::login(&db, request.into_inner()).await
}

/// Get authenticated user, permissions and roles
#[utoipa::path(
    tag = "Authentication",
    security(("token" = [])),
    responses(
        responses::auth::Authenticated,
        Unauthorized,
        InternalServerError,
    ),
)]
#[get("/user")]
pub async fn authenticate(auth: Auth) -> impl Responder {
    services::auth::authenticate(auth).await
}

/// Logout by request authorization token
#[utoipa::path(
    tag = "Authentication",
    security(("token" = [])),
    responses(
        Ok,
        Unauthorized,
        InternalServerError,
    ),
)]
#[delete("/logout")]
pub async fn logout(db: Data<DatabaseConnection>, auth: Auth) -> impl Responder {
    services::auth::logout(&db, auth).await
}
