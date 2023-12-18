use sea_orm::prelude::*;
use sea_orm::Set;

use crate::common::log;
use crate::models::{roles, Id};
use crate::requests::role::{RoleStoreRequest, RoleUpdateRequest};

pub async fn find<I: Into<Id>>(db: &DatabaseConnection, id: I) -> Option<roles::Model> {
    let id: Id = id.into();

    match roles::Entity::find_by_id(id).one(db).await {
        Ok(role) => role,
        Err(e) => {
            log::error!(find, "{}", e);

            None
        }
    }
}

pub async fn store(
    db: &DatabaseConnection,
    request: RoleStoreRequest,
) -> Result<roles::Model, DbErr> {
    let role = roles::Model {
        id: Uuid::new_v4().into(),
        code: request.code.to_uppercase().replace(" ", "_"),
        name: request.name.to_lowercase(),
    };

    roles::ActiveModel::from(role).insert(db).await
}

pub async fn update(
    db: &DatabaseConnection,
    role: roles::Model,
    request: RoleUpdateRequest,
) -> Result<roles::Model, DbErr> {
    if role.name == request.name {
        return Ok(role);
    }

    let mut model = roles::ActiveModel::from(role.clone());

    if role.name != request.name {
        model.name = Set(request.name);
    }

    model.update(db).await
}

pub async fn delete(db: &DatabaseConnection, role: roles::Model) -> Result<roles::Model, DbErr> {
    roles::ActiveModel::from(role.clone()).delete(db).await?;

    Ok(role)
}
