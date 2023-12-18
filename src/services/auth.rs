use std::collections::HashMap;

use actix_web::HttpResponse;
use sea_orm::DatabaseConnection;
use serde_json::json;

use crate::common::hash::Hash;
use crate::common::{base58, hash, log};
use crate::dao::{self, user};
use crate::middlewares::auth::Auth;
use crate::requests::auth::Login;
use crate::responses::user::UserOAS;

pub async fn login(db: &DatabaseConnection, request: Login) -> HttpResponse {
    let mut validation = HashMap::new();
    let email_or_username = request.email_or_username.trim().to_lowercase();
    let password = request.password;
    let mut user = None;

    if email_or_username.is_empty() {
        validation.insert(
            "email_or_username",
            vec!["field email or username is required"],
        );
    } else {
        let mut errors = vec![];

        user = user::find_by_email_or_username(db, email_or_username.clone()).await;

        if user.is_none() {
            errors.push("email or username doesn't exist");
        }

        if !errors.is_empty() {
            validation.insert("email_or_username", errors);
        }
    }

    if password.is_empty() {
        validation.insert("password", vec!["password field is required"]);
    } else {
        let mut errors = vec![];

        if let Some((user, _, _)) = user.clone() {
            let hashed = Hash::from(user.password);

            if !hash::verify(hashed, user.id.to_string(), password) {
                errors.push("wrong password");
            }
        }

        if !errors.is_empty() {
            validation.insert("password", errors);
        }
    }

    if !validation.is_empty() {
        return HttpResponse::UnprocessableEntity().json(json!({
            "errors": validation,
        }));
    }

    let (user, permissions, roles) = user.unwrap();

    match dao::auth::generate(db, &user, None).await {
        Err(e) => {
            log::error!(services::auth::login, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        }
        Ok(token) => {
            let response = json!({
                "token": base58::to_string(token.id.as_bytes()),
                "user": UserOAS::from((user, permissions, roles)),
            });

            HttpResponse::Ok().json(response)
        }
    }
}

pub async fn authenticate(auth: Auth) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "user": UserOAS::from((auth.user, auth.permissions, auth.roles)),
    }))
}

pub async fn logout(db: &DatabaseConnection, auth: Auth) -> HttpResponse {
    match dao::auth::delete(db, auth.user.id).await {
        Err(e) => {
            log::error!(services::auth::logout, "{}", e);

            HttpResponse::InternalServerError().json(json!({
                "message": e.to_string(),
            }))
        }
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
