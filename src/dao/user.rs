use sea_orm::prelude::*;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use sea_query::Condition;

use crate::common::hash;
use crate::common::log;
use crate::common::time;
use crate::models::permission_user;
use crate::models::permissions;
use crate::models::role_user;
use crate::models::roles;
use crate::models::{users, Id};
use crate::requests::user::{UserStoreRequest, UserUpdateGeneralInformationRequest};

pub async fn find<I: Into<Id>>(
    db: &DatabaseConnection,
    id: I,
) -> Option<(users::Model, Vec<permissions::Model>, Vec<roles::Model>)> {
    let id: Id = id.into();
    let user = users::Entity::find_by_id(id)
        .filter(users::Column::DeletedAt.is_null())
        .one(db)
        .await;

    if let Err(e) = &user {
        log::error!(find, "{}", e);

        return None;
    }

    let user = user.unwrap();

    if user.is_none() {
        return None;
    }

    let user = user.unwrap();
    let permissions = permissions::Entity::find()
        .find_with_related(permission_user::Entity)
        .filter(permission_user::Column::UserId.eq(user.id.clone()))
        .all(db)
        .await;

    if let Err(e) = permissions {
        log::error!(find, "{}", e);

        return None;
    }

    let roles = roles::Entity::find()
        .find_with_related(role_user::Entity)
        .filter(users::Column::Id.eq(user.id.clone()))
        .all(db)
        .await;

    if let Err(e) = roles {
        log::error!(find, "{}", e);

        return None;
    }

    let permissions = permissions
        .unwrap()
        .iter()
        .map(|(permission, _)| permission.clone())
        .collect();

    let roles = roles
        .unwrap()
        .iter()
        .map(|(role, _)| role.clone())
        .collect();

    Some((user, permissions, roles))
}

pub async fn find_by_email_or_username(
    db: &DatabaseConnection,
    email_or_username: String,
) -> Option<(users::Model, Vec<permissions::Model>, Vec<roles::Model>)> {
    let email_or_username = email_or_username.trim().to_lowercase();
    let user = users::Entity::find()
        .filter(users::Column::DeletedAt.is_null())
        .filter(
            Condition::any()
                .add(users::Column::Email.eq(email_or_username.clone()))
                .add(users::Column::Username.eq(email_or_username.clone())),
        )
        .one(db)
        .await;

    if let Err(e) = &user {
        log::error!(find_by_email_or_username, "{}", e);

        return None;
    }

    let user = user.unwrap();

    if user.is_none() {
        return None;
    }

    let user = user.unwrap();
    let permissions = permissions::Entity::find()
        .find_with_related(permission_user::Entity)
        .filter(permission_user::Column::UserId.eq(user.id.clone()))
        .all(db)
        .await;

    if let Err(e) = permissions {
        log::error!(find_by_email_or_username, "{}", e);

        return None;
    }

    let roles = roles::Entity::find()
        .find_with_related(role_user::Entity)
        .filter(role_user::Column::UserId.eq(user.id.clone()))
        .all(db)
        .await;

    if let Err(e) = roles {
        log::error!(find_by_email_or_username, "{}", e);

        return None;
    }

    let permissions = permissions
        .unwrap()
        .iter()
        .map(|(permission, _)| permission.clone())
        .collect();

    let roles = roles
        .unwrap()
        .iter()
        .map(|(role, _)| role.clone())
        .collect();

    Some((user, permissions, roles))
}

pub async fn store(
    db: &DatabaseConnection,
    request: UserStoreRequest,
) -> Result<(users::Model, Vec<permissions::Model>, Vec<roles::Model>), DbErr> {
    let tx = db.begin().await?;
    let id = Uuid::new_v4();
    let user = users::Model {
        id: id.into(),
        name: request.name.trim().to_lowercase(),
        email: request.email.trim().to_lowercase(),
        email_verified_at: None,
        username: request.username.trim().to_lowercase(),
        password: hash::make(id, request.password).to_string(),
        profile_photo_id: None,
        created_at: time::now(),
        updated_at: time::now(),
        deleted_at: None,
    };

    let user = users::ActiveModel::from(user).insert(&tx).await;

    if let Err(e) = user {
        tx.rollback().await?;

        return Err(e);
    }

    let user = user?;
    let permissions = permissions::Entity::find()
        .filter(permissions::Column::Id.is_in(request.permissions))
        .all(&tx)
        .await;

    if let Err(e) = permissions {
        tx.rollback().await?;

        return Err(e);
    }

    let permissions = permissions?;
    let permission_users = permissions
        .clone()
        .iter()
        .map(|permission| permission_user::Model {
            id: Uuid::new_v4().into(),
            user_id: user.id.clone(),
            permission_id: permission.id.clone(),
        })
        .map(|permission_user| permission_user::ActiveModel::from(permission_user))
        .collect::<Vec<_>>();

    let permission_users = permission_user::Entity::insert_many(permission_users);

    if let Err(e) = permission_users.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    let roles = roles::Entity::find()
        .filter(roles::Column::Id.is_in(request.roles))
        .all(&tx)
        .await;

    if let Err(e) = roles {
        tx.rollback().await?;

        return Err(e);
    }

    let roles = roles?;
    let role_users = roles
        .clone()
        .iter()
        .map(|role| role_user::Model {
            id: Uuid::new_v4().into(),
            user_id: user.id.clone(),
            role_id: role.id.clone(),
        })
        .map(|role_user| role_user::ActiveModel::from(role_user))
        .collect::<Vec<_>>();

    let role_users = role_user::Entity::insert_many(role_users);

    if let Err(e) = role_users.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    tx.commit().await?;

    Ok((user, permissions, roles))
}

pub async fn update_general_information(
    db: &DatabaseConnection,
    user: users::Model,
    request: UserUpdateGeneralInformationRequest,
) -> Result<(users::Model, Vec<permissions::Model>, Vec<roles::Model>), DbErr> {
    let tx = db.begin().await?;
    let name = request.name.trim().to_lowercase();
    let email = request.email.trim().to_lowercase();
    let username = request.username.trim().to_lowercase();
    let mut model = users::ActiveModel::from(user.clone());

    if user.name != name {
        model.name = Set(name);
    }

    if user.email != email {
        model.email = Set(email);
    }

    if user.username != username {
        model.username = Set(username);
    }

    if let Some(at) = user.email_verified_at {
        model.email_verified_at = Set(Some(at));
    }

    if let Some(id) = user.profile_photo_id {
        model.profile_photo_id = Set(Some(id));
    }

    model.updated_at = Set(time::now());

    let user = model.update(&tx).await;

    if let Err(e) = user {
        tx.rollback().await?;

        return Err(e);
    }

    let user = user?;
    let permissions = permissions::Entity::find()
        .filter(permissions::Column::Id.is_in(request.permissions))
        .all(&tx)
        .await;

    if let Err(e) = permissions {
        tx.rollback().await?;

        return Err(e);
    }

    let permissions = permissions?;
    let delete = permission_user::Entity::delete_many()
        .filter(permission_user::Column::UserId.eq(user.id.clone()));

    if let Err(e) = delete.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    let permission_user = permissions
        .clone()
        .iter()
        .map(|permission| {
            permission_user::ActiveModel::from(permission_user::Model {
                id: Uuid::new_v4().into(),
                permission_id: permission.id.clone(),
                user_id: user.id.clone(),
            })
        })
        .collect::<Vec<_>>();

    let permission_user = permission_user::Entity::insert_many(permission_user);

    if let Err(e) = permission_user.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    let roles = roles::Entity::find()
        .filter(roles::Column::Id.is_in(request.roles))
        .all(&tx)
        .await;

    if let Err(e) = roles {
        tx.rollback().await?;

        return Err(e);
    }

    let roles = roles?;
    let delete =
        role_user::Entity::delete_many().filter(role_user::Column::UserId.eq(user.id.clone()));

    if let Err(e) = delete.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    let role_user = roles
        .clone()
        .iter()
        .map(|role| {
            role_user::ActiveModel::from(role_user::Model {
                id: Uuid::new_v4().into(),
                role_id: role.id.clone(),
                user_id: user.id.clone(),
            })
        })
        .collect::<Vec<_>>();

    let role_user = role_user::Entity::insert_many(role_user);

    if let Err(e) = role_user.exec(&tx).await {
        tx.rollback().await?;

        return Err(e);
    }

    Ok((user, permissions, roles))
}

pub async fn update_password(
    db: &DatabaseConnection,
    user: users::Model,
    password: String,
) -> Result<users::Model, DbErr> {
    let password = hash::make(user.id.clone(), password);
    let mut model = users::ActiveModel::from(user);
    model.password = Set(password.to_string());
    model.updated_at = Set(time::now());
    model.update(db).await
}

pub async fn delete(db: &DatabaseConnection, user: users::Model) -> Result<users::Model, DbErr> {
    let mut model = users::ActiveModel::from(user);
    model.deleted_at = Set(Some(time::now()));
    model.update(db).await
}

pub async fn email_exist<T: ToString>(db: &DatabaseConnection, email: T) -> Result<bool, DbErr> {
    let email = email.to_string().trim().to_lowercase();
    let exist = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .count(db)
        .await;

    if let Err(e) = exist {
        log::error!(email_exist, "{}", e);

        return Err(e);
    }

    Ok(exist? > 0)
}

pub async fn email_exist_except<T: ToString, I: Into<Id>>(
    db: &DatabaseConnection,
    email: T,
    id: I,
) -> Result<bool, DbErr> {
    let email = email.to_string().trim().to_lowercase();
    let exist = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .filter(users::Column::Id.ne(id.into()))
        .count(db)
        .await;

    if let Err(e) = exist {
        log::error!(email_exist, "{}", e);

        return Err(e);
    }

    Ok(exist? > 0)
}

pub async fn username_exist<T: ToString>(
    db: &DatabaseConnection,
    username: T,
) -> Result<bool, DbErr> {
    let username = username.to_string().trim().to_lowercase();
    let exist = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .count(db)
        .await;

    if let Err(e) = exist {
        log::error!(username_exist, "{}", e);

        return Err(e);
    }

    Ok(exist? > 0)
}

pub async fn username_exist_except<T: ToString, I: Into<Id>>(
    db: &DatabaseConnection,
    username: T,
    id: I,
) -> Result<bool, DbErr> {
    let username = username.to_string().trim().to_lowercase();
    let exist = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .filter(users::Column::Id.ne(id.into()))
        .count(db)
        .await;

    if let Err(e) = exist {
        log::error!(username_exist, "{}", e);

        return Err(e);
    }

    Ok(exist? > 0)
}
