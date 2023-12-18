use sea_orm::prelude::*;
use sea_orm::Set;

use crate::common::log;
use crate::models::{permissions, Id};
use crate::requests::permission::{PermissionStoreRequest, PermissionUpdateRequest};

pub async fn find<I: Into<Id>>(db: &DatabaseConnection, id: I) -> Option<permissions::Model> {
    let id: Id = id.into();

    match permissions::Entity::find_by_id(id).one(db).await {
        Ok(permission) => permission,
        Err(e) => {
            log::error!(find, "{}", e);

            None
        }
    }
}

pub async fn store(
    db: &DatabaseConnection,
    request: PermissionStoreRequest,
) -> Result<permissions::Model, DbErr> {
    let permission = permissions::Model {
        id: Uuid::new_v4().into(),
        code: request.code.to_uppercase().replace(" ", "_"),
        name: request.name.to_lowercase(),
    };

    permissions::ActiveModel::from(permission).insert(db).await
}

pub async fn update(
    db: &DatabaseConnection,
    permission: permissions::Model,
    request: PermissionUpdateRequest,
) -> Result<permissions::Model, DbErr> {
    if permission.name == request.name {
        return Ok(permission);
    }

    let mut model = permissions::ActiveModel::from(permission.clone());

    if permission.name != request.name {
        model.name = Set(request.name);
    }

    model.update(db).await
}

pub async fn delete(
    db: &DatabaseConnection,
    permission: permissions::Model,
) -> Result<permissions::Model, DbErr> {
    permissions::ActiveModel::from(permission.clone())
        .delete(db)
        .await?;

    Ok(permission)
}
