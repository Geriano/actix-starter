use learning_management_system::{
    common::{hash, time},
    models::{permission_user, permissions, role_user, roles, users},
};
use sea_orm::prelude::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const PERMISSIONS: [&str; 3] = ["user", "permission", "role"];
const ABILITIES: [&str; 4] = ["create", "read", "update", "delete"];
const ROLES: [&str; 2] = ["superuser", "admin"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let (mut permissions, mut roles) = (vec![], vec![]);

        for permission in PERMISSIONS {
            for ability in ABILITIES {
                let permission = permissions::Model {
                    id: Uuid::new_v4().into(),
                    code: format!("{}_{}", ability, permission).to_uppercase(),
                    name: format!("{} {}", ability, permission).to_lowercase(),
                };

                permissions.push(permission);
            }
        }

        for role in ROLES {
            let role = roles::Model {
                id: Uuid::new_v4().into(),
                code: role.to_uppercase(),
                name: role.to_lowercase(),
            };

            roles.push(role);
        }

        let query = permissions::Entity::insert_many(
            permissions
                .iter()
                .map(|permission| permissions::ActiveModel::from(permission.clone()))
                .collect::<Vec<_>>(),
        );

        query.exec(manager.get_connection()).await?;

        let query = roles::Entity::insert_many(
            roles
                .iter()
                .map(|role| roles::ActiveModel::from(role.clone()))
                .collect::<Vec<_>>(),
        );

        query.exec(manager.get_connection()).await?;

        let id = Uuid::new_v4();
        let user = users::Model {
            id: id.clone().into(),
            name: "root".to_owned(),
            email: "root@local".to_owned(),
            username: "root".to_owned(),
            password: hash::make(id.to_string(), "LetMe!nM4te").to_string(),
            profile_photo_id: None,
            email_verified_at: None,
            created_at: time::now(),
            updated_at: time::now(),
            deleted_at: None,
        };

        users::ActiveModel::from(user)
            .insert(manager.get_connection())
            .await?;

        let (mut permission_users, mut role_users) = (vec![], vec![]);

        for permission in permissions {
            let permission_user = permission_user::Model {
                id: Uuid::new_v4().into(),
                permission_id: permission.id.clone(),
                user_id: id.clone().into(),
            };

            permission_users.push(permission_user);
        }

        for role in roles {
            let role_user = role_user::Model {
                id: Uuid::new_v4().into(),
                role_id: role.id.clone(),
                user_id: id.clone().into(),
            };

            role_users.push(role_user);
        }

        let query = permission_user::Entity::insert_many(
            permission_users
                .iter()
                .map(|permission_user| permission_user::ActiveModel::from(permission_user.clone()))
                .collect::<Vec<_>>(),
        );

        query.exec(manager.get_connection()).await?;

        let query = role_user::Entity::insert_many(
            role_users
                .iter()
                .map(|role_user| role_user::ActiveModel::from(role_user.clone()))
                .collect::<Vec<_>>(),
        );

        query.exec(manager.get_connection()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        users::Entity::delete_many()
            .filter(users::Column::Username.eq("root"))
            .exec(manager.get_connection())
            .await?;

        let (mut permissions, mut roles) = (vec![], vec![]);

        for permission in PERMISSIONS {
            for ability in ABILITIES {
                permissions.push(format!("{}_{}", ability, permission).to_uppercase());
            }
        }

        for role in ROLES {
            roles.push(role.to_uppercase());
        }

        permissions::Entity::delete_many()
            .filter(permissions::Column::Code.is_in(permissions))
            .exec(manager.get_connection())
            .await?;

        roles::Entity::delete_many()
            .filter(roles::Column::Code.is_in(roles))
            .exec(manager.get_connection())
            .await?;

        Ok(())
    }
}
