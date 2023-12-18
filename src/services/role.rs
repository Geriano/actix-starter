use std::collections::HashMap;

use actix_web::HttpResponse;
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use sea_query::Condition;

use crate::common::log;
use crate::dao;
use crate::models::{roles, Id};
use crate::requests::role::{RoleStoreRequest, RoleUpdateRequest};
use crate::requests::PaginationRequest;
use crate::responses::role::{RoleOAS, RolePaginationResponse};
use crate::responses::{CreatedWithId, InternalServerError, Ok, UnprocessableEntity};

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<roles::Column>,
) -> HttpResponse {
    let mut query = roles::Entity::find();

    if let Some(search) = request.search() {
        let search = format!("%{}%", search.to_lowercase());

        query = query.filter(
            Condition::any()
                .add(roles::Column::Code.like(search.clone()))
                .add(roles::Column::Name.like(search)),
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
    let roles = query
        .limit(request.limit())
        .offset(request.offset())
        .order_by(request.order(roles::Column::Code), request.sort())
        .all(db);

    match roles.await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(roles) => RolePaginationResponse {
            total,
            page: total / request.limit(),
            data: roles.iter().map(RoleOAS::from).collect(),
        }
        .into(),
    }
}

pub async fn store(db: &DatabaseConnection, request: RoleStoreRequest) -> HttpResponse {
    let mut validation = HashMap::new();
    let code = request.code.trim().to_uppercase();
    let name = request.name.trim().to_lowercase();
    let mut errors = vec![];

    if code.is_empty() {
        errors.push("Code field is required");
    }

    if name.is_empty() {
        validation.insert("name", vec!["Name field is required"]);
    }

    let exist = roles::Entity::find()
        .filter(roles::Column::Code.eq(code.clone()))
        .count(db)
        .await;

    if let Err(e) = exist {
        log::error!(store, "{}", e);

        return InternalServerError {
            message: e.to_string(),
        }
        .into();
    }

    if exist.unwrap() > 0 {
        errors.push("Code already exist");
    }

    if !errors.is_empty() {
        validation.insert("code", errors);
    }

    if !validation.is_empty() {
        return UnprocessableEntity { errors: validation }.into();
    }

    match dao::role::store(db, request).await {
        Err(e) => {
            log::error!(store, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(role) => CreatedWithId {
            id: role.id,
            message: format!("Role {} has been created", role.code),
        }
        .into(),
    }
}

pub async fn show<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    match dao::role::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(role) => RoleOAS::from(role).into(),
    }
}

pub async fn update<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: RoleUpdateRequest,
) -> HttpResponse {
    let id: Id = id.into();
    let mut validation = HashMap::new();
    let name = request.name.trim().to_lowercase();

    if name.is_empty() {
        validation.insert("name", vec!["Name field is required"]);
    }

    if !validation.is_empty() {
        return UnprocessableEntity { errors: validation }.into();
    }

    let role = dao::role::find(db, id).await;

    if role.is_none() {
        return HttpResponse::NotFound().finish();
    }

    match dao::role::update(db, role.unwrap(), request).await {
        Err(e) => {
            log::error!(update, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(role) => RoleOAS::from(role).into(),
    }
}

pub async fn delete<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    let role = dao::role::find(db, id).await;

    if role.is_none() {
        return HttpResponse::NotFound().finish();
    }

    match dao::role::delete(db, role.unwrap()).await {
        Err(e) => {
            log::error!(delete, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(role) => Ok {
            message: format!("Role {} has been deleted", role.code),
        }
        .into(),
    }
}
