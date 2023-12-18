use std::collections::HashMap;

use actix_web::HttpResponse;
use sea_orm::{prelude::*, QueryOrder, QuerySelect};
use sea_query::Condition;

use crate::common::log;
use crate::dao;
use crate::models::{permissions, Id};
use crate::requests::permission::{PermissionStoreRequest, PermissionUpdateRequest};
use crate::requests::PaginationRequest;
use crate::responses::permission::{PermissionOAS, PermissionPaginationResponse};
use crate::responses::{CreatedWithId, InternalServerError, Ok, UnprocessableEntity};

pub async fn paginate(
    db: &DatabaseConnection,
    request: PaginationRequest<permissions::Column>,
) -> HttpResponse {
    let mut query = permissions::Entity::find();

    if let Some(search) = request.search() {
        let search = format!("%{}%", search.to_lowercase());

        query = query.filter(
            Condition::any()
                .add(permissions::Column::Code.like(search.clone()))
                .add(permissions::Column::Name.like(search)),
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
    let permissions = query
        .limit(request.limit())
        .offset(request.offset())
        .order_by(request.order(permissions::Column::Code), request.sort())
        .all(db);

    match permissions.await {
        Err(e) => {
            log::error!(paginate, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(permissions) => PermissionPaginationResponse {
            total,
            page: total / request.limit(),
            data: permissions.iter().map(PermissionOAS::from).collect(),
        }
        .into(),
    }
}

pub async fn store(db: &DatabaseConnection, request: PermissionStoreRequest) -> HttpResponse {
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

    let exist = permissions::Entity::find()
        .filter(permissions::Column::Code.eq(code.clone()))
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

    match dao::permission::store(db, request).await {
        Err(e) => {
            log::error!(store, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(permission) => CreatedWithId {
            id: permission.id,
            message: format!("Permission {} has been created", permission.code),
        }
        .into(),
    }
}

pub async fn show<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    match dao::permission::find(db, id).await {
        None => HttpResponse::NotFound().finish(),
        Some(permission) => PermissionOAS::from(permission).into(),
    }
}

pub async fn update<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
    request: PermissionUpdateRequest,
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

    let permission = dao::permission::find(db, id).await;

    if permission.is_none() {
        return HttpResponse::NotFound().finish();
    }

    match dao::permission::update(db, permission.unwrap(), request).await {
        Err(e) => {
            log::error!(update, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(permission) => PermissionOAS::from(permission).into(),
    }
}

pub async fn delete<I: Into<Id>>(db: &DatabaseConnection, id: I) -> HttpResponse {
    let permission = dao::permission::find(db, id).await;

    if permission.is_none() {
        return HttpResponse::NotFound().finish();
    }

    match dao::permission::delete(db, permission.unwrap()).await {
        Err(e) => {
            log::error!(delete, "{}", e);

            InternalServerError {
                message: e.to_string(),
            }
            .into()
        }
        Ok(permission) => Ok {
            message: format!("Permission {} has been deleted", permission.code),
        }
        .into(),
    }
}
