use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS users (
                    id VARCHAR(36) NOT NULL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    email VARCHAR(255) NOT NULL,
                    email_verified_at TIMESTAMP NULL DEFAULT NULL,
                    username VARCHAR(255) NOT NULL,
                    password VARCHAR(255) NOT NULL,
                    profile_photo_id VARCHAR(36) NULL DEFAULT NULL,
                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    deleted_at TIMESTAMP NULL DEFAULT NULL
                )",
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::EmailVerifiedAt)
                            .timestamp()
                            .null()
                            .default(None as Option<String>),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(
                        ColumnDef::new(User::ProfilePhotoId)
                            .string()
                            .null()
                            .default(None as Option<String>),
                    )
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()"),
                    )
                    .col(
                        ColumnDef::new(User::DeletedAt)
                            .timestamp()
                            .null()
                            .default(None as Option<String>),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::Email)
                    .name("idx_users_email")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::EmailVerifiedAt)
                    .name("idx_users_email_verified_at")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::Username)
                    .name("idx_users_username")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::CreatedAt)
                    .name("idx_users_created_at")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::UpdatedAt)
                    .name("idx_users_updated_at")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(User::Table)
                    .col(User::DeletedAt)
                    .name("idx_users_deleted_at")
                    .take(),
            )
            .await?;

        // let id: Id = uuid::Uuid::new_v4().into();
        // let name = "root".to_owned();
        // let email = "root@local.app".to_owned();
        // let email_verified_at = Some(nightmare_common::time::now());
        // let username = "root".to_owned();
        // let password = nightmare_common::hash::make(id.clone(), "LetMe!nM4te").to_string();
        // let profile_photo_id: Option<String> = None;
        // let created_at = nightmare_common::time::now();
        // let updated_at = created_at.clone();
        // let deleted_at = None;
        // let user = nightmare_common::models::users::Model {
        //     id, name, email, email_verified_at, username, password,
        //     profile_photo_id, created_at, updated_at, deleted_at
        // };

        // let user = nightmare_common::models::users::ActiveModel::from(user);
        // let query = nightmare_common::models::users::Entity::insert(user);

        // nightmare_common::log::debug!(up, "query: {:?}", query.build(manager.get_database_backend()).to_string());
        // query.exec(manager.get_connection()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).take())
            .await
    }
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Name,
    Email,
    EmailVerifiedAt,
    Username,
    Password,
    ProfilePhotoId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
