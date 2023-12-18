use std::collections::HashMap;

use actix_web::HttpResponse;
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use sea_query::Condition;

use crate::common::hash::Hash;
use crate::common::{hash, log};
use crate::dao;
use crate::models::{permission_user, permissions, role_user, roles, users, Id};
use crate::requests::user::{
    UserStoreRequest, UserUpdateGeneralInformationRequest, UserUpdatePasswordRequest,
};
use crate::requests::PaginationRequest;
use crate::responses::permission::PermissionOAS;
use crate::responses::role::RoleOAS;
use crate::responses::user::{UserOAS, UserPaginationResponse};
use crate::responses::{InternalServerError, Ok, UnprocessableEntity};

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<users::Column>,
) -> HttpResponse {
    let mut query = users::Entity::find().filter(users::Column::DeletedAt.is_null());

    if let Some(search) = request.search() {
        let search = format!("%{}%", search.to_lowercase());

        query = query.filter(
            Condition::any()
                .add(users::Column::Name.like(search.clone()))
                .add(users::Column::Email.like(search.clone()))
                .add(users::Column::Username.like(search)),
        );
    }

    let total = query.clone().count(db).await;

    if let Err(e) = total {
        log::error!(paginate, "{}", e);

        return InternalServerError {
            message: e.to_string(),
        }
        .into();
    }

    let total = total.unwrap();
    let users = query
        .limit(request.limit())
        .offset(request.offset())
        .order_by(request.order(users::Column::CreatedAt), request.sort())
        .all(db);

    match users.await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(users) => {
            let permission_user = permission_user::Entity::find()
                .find_with_related(permissions::Entity)
                .filter(
                    permission_user::Column::UserId.is_in(
                        users
                            .iter()
                            .map(|user| user.id.clone())
                            .collect::<Vec<Id>>(),
                    ),
                )
                .all(db)
                .await;

            if let Err(e) = permission_user {
                log::error!(paginate, "{}", e);

                return InternalServerError {
                    message: e.to_string(),
                }
                .into();
            }

            let permission_user = permission_user.unwrap();

            let role_user = role_user::Entity::find()
                .find_with_related(roles::Entity)
                .filter(
                    role_user::Column::UserId.is_in(
                        users
                            .iter()
                            .map(|user| user.id.clone())
                            .collect::<Vec<Id>>(),
                    ),
                )
                .all(db)
                .await;

            if let Err(e) = role_user {
                log::error!(paginate, "{}", e);

                return InternalServerError {
                    message: e.to_string(),
                }
                .into();
            }

            let role_user = role_user.unwrap();

            let users = users
                .iter()
                .map(|user| {
                    let mut user = UserOAS::from(user);

                    for (permission_user, permissions) in permission_user.clone() {
                        if permission_user.user_id == user.id {
                            user.permissions
                                .extend(permissions.iter().map(PermissionOAS::from));
                        }
                    }

                    for (role_user, roles) in role_user.clone() {
                        if role_user.user_id == user.id {
                            user.roles.extend(roles.iter().map(RoleOAS::from));
                        }
                    }

                    user
                })
                .collect();

            UserPaginationResponse {
                total,
                page: total / request.limit(),
                data: users,
            }
            .into()
        }
    }
}

pub async fn store(db: &DatabaseConnection, request: UserStoreRequest) -> HttpResponse {
    let mut validation = HashMap::new();
    let name = request.name.trim().to_lowercase();
    let email = request.email.trim().to_lowercase();
    let username = request.username.trim().to_lowercase();
    let password = request.password.trim().to_lowercase();
    let permissions = request.permissions.clone();
    let roles = request.roles.clone();

    if name.is_empty() {
        validation.insert("name", vec!["Name field is required"]);
    }

    if email.is_empty() {
        validation.insert("email", vec!["Email field is required"]);
    } else {
        let mut errors = vec![];
        let exist = dao::user::email_exist(db, email.clone()).await;

        if let Err(e) = exist {
            log::error!(store, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if exist.unwrap() {
            errors.push("Email already exist");
        }

        if !email.contains("@") {
            errors.push("Email is invalid");
        }

        if !errors.is_empty() {
            validation.insert("email", errors);
        }
    }

    if username.is_empty() {
        validation.insert("username", vec!["Username field is required"]);
    } else {
        let mut errors = vec![];
        let exist = dao::user::username_exist(db, username).await;

        if let Err(e) = exist {
            log::error!(store, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if exist.unwrap() {
            errors.push("Username already exist");
        }

        if !errors.is_empty() {
            validation.insert("username", errors);
        }
    }

    if password.is_empty() {
        validation.insert("password", vec!["Password field is required"]);
    } else {
        let mut errors = vec![];

        if password.len() < 6 {
            errors.push("Password must be at least 6 characters");
        }

        if password.to_lowercase() == password {
            errors.push("Password must contain at least 1 uppercase character");
        }

        if !password.chars().any(|c| c.is_digit(10)) {
            errors.push("Password must contain at least 1 digit");
        }

        if !password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("Password must contain at least 1 special character");
        }

        if !errors.is_empty() {
            validation.insert("password", errors);
        }
    }

    if !permissions.is_empty() {
        let permissions = permissions::Entity::find()
            .filter(permissions::Column::Code.is_in(permissions))
            .all(db)
            .await;

        if let Err(e) = permissions {
            log::error!(store, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if request.permissions.len() != permissions.unwrap().len() {
            validation.insert("permissions", vec!["Some permissions are invalid"]);
        }

        if !validation.is_empty() {
            return UnprocessableEntity { errors: validation }.into();
        }
    }

    if !roles.is_empty() {
        let roles = roles::Entity::find()
            .filter(roles::Column::Code.is_in(roles))
            .all(db)
            .await;

        if let Err(e) = roles {
            log::error!(store, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if request.roles.len() != roles.unwrap().len() {
            validation.insert("roles", vec!["Some roles are invalid"]);
        }

        if !validation.is_empty() {
            return UnprocessableEntity { errors: validation }.into();
        }
    }

    if !validation.is_empty() {
        return UnprocessableEntity { errors: validation }.into();
    }

    match dao::user::store(db, request).await {
        Err(e) => {
            log::error!(store, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(user) => UserOAS::from(user).into(),
    }
}

pub async fn show<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    match dao::user::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(user) => UserOAS::from(user).into(),
    }
}

pub async fn update_general_information<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: UserUpdateGeneralInformationRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let (user, _, _) = user.unwrap();
    let mut validation = HashMap::new();
    let name = request.name.trim().to_lowercase();
    let email = request.email.trim().to_lowercase();
    let username = request.username.trim().to_lowercase();
    let permissions = request.permissions.clone();
    let roles = request.roles.clone();

    if name.is_empty() {
        validation.insert("name", vec!["Name field is required"]);
    }

    if email.is_empty() {
        validation.insert("email", vec!["Email field is required"]);
    } else {
        let mut errors = vec![];
        let exist = dao::user::email_exist_except(db, email.clone(), user.id.clone()).await;

        if let Err(e) = exist {
            log::error!(update_general_information, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if exist.unwrap() && email != user.email {
            errors.push("Email already exist");
        }

        if !email.contains("@") {
            errors.push("Email is invalid");
        }

        if !errors.is_empty() {
            validation.insert("email", errors);
        }
    }

    if username.is_empty() {
        validation.insert("username", vec!["Username field is required"]);
    } else {
        let mut errors = vec![];
        let exist = dao::user::username_exist_except(db, username.clone(), user.id.clone()).await;

        if let Err(e) = exist {
            log::error!(update_general_information, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if exist.unwrap() && username != user.username {
            errors.push("Username already exist");
        }

        if !errors.is_empty() {
            validation.insert("username", errors);
        }
    }

    if !permissions.is_empty() {
        let mut errors = vec![];
        let permissions = permissions::Entity::find()
            .filter(permissions::Column::Code.is_in(permissions))
            .all(db)
            .await;

        if let Err(e) = permissions {
            log::error!(update_general_information, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if request.permissions.len() != permissions.unwrap().len() {
            errors.push("Some permissions are invalid");
        }

        if !errors.is_empty() {
            validation.insert("permissions", errors);
        }
    }

    if !roles.is_empty() {
        let mut errors = vec![];
        let roles = roles::Entity::find()
            .filter(roles::Column::Code.is_in(roles))
            .all(db)
            .await;

        if let Err(e) = roles {
            log::error!(update_general_information, "{}", e);

            return InternalServerError {
                message: e.to_string(),
            }
            .into();
        }

        if request.roles.len() != roles.unwrap().len() {
            errors.push("Some roles are invalid");
        }

        if !errors.is_empty() {
            validation.insert("roles", errors);
        }
    }

    if !validation.is_empty() {
        return UnprocessableEntity { errors: validation }.into();
    }

    match dao::user::update_general_information(db, user, request).await {
        Err(e) => {
            log::error!(update, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(user) => UserOAS::from(user).into(),
    }
}

pub async fn update_password<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: UserUpdatePasswordRequest,
) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let (user, _, _) = user.unwrap();
    let mut validation = HashMap::new();
    let current_password = request.current_password.clone();
    let password_confirmation = request.password_confirmation.clone();
    let new_password = request.new_password.clone();

    if current_password.is_empty() {
        validation.insert(
            "current_password",
            vec!["Current password field is required"],
        );
    } else if !hash::verify(
        Hash::from(user.password.clone()),
        user.id.clone(),
        current_password,
    ) {
        validation.insert("current_password", vec!["Current password is invalid"]);
    }

    if new_password.is_empty() {
        validation.insert("new_password", vec!["New password field is required"]);
    } else {
        let mut errors = vec![];

        if new_password.len() < 6 {
            errors.push("New password must be at least 6 characters");
        }

        if new_password.to_lowercase() == new_password {
            errors.push("New password must contain at least 1 uppercase character");
        }

        if !new_password.chars().any(|c| c.is_digit(10)) {
            errors.push("New password must contain at least 1 digit");
        }

        if !new_password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("New password must contain at least 1 special character");
        }

        if !errors.is_empty() {
            validation.insert("new_password", errors);
        }
    }

    if password_confirmation.is_empty() {
        validation.insert(
            "password_confirmation",
            vec!["Password confirmation field is required"],
        );
    } else if password_confirmation != new_password {
        validation.insert(
            "password_confirmation",
            vec!["Password confirmation must match new password"],
        );
    }

    if !validation.is_empty() {
        return UnprocessableEntity { errors: validation }.into();
    }

    match dao::user::update_password(db, user, new_password).await {
        Err(e) => {
            log::error!(update_password, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(user) => Ok {
            message: format!("Password for user {} has been updated", user.username),
        }
        .into(),
    }
}

pub async fn delete<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    let user = dao::user::find(db, id).await;

    if user.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let (user, _, _) = user.unwrap();

    match dao::user::delete(db, user).await {
        Err(e) => {
            log::error!(delete, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(user) => Ok {
            message: format!("User {} has been deleted", user.username),
        }
        .into(),
    }
}
