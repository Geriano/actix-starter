use sea_orm_migration::prelude::*;

#[allow(unused_imports)]
use crate::{m20230902_024725_create_users::User, m20230902_024928_create_permissions::Permission};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS permission_user (
                    id VARCHAR(36) NOT NULL PRIMARY KEY,
                    permission_id VARCHAR(36) NOT NULL,
                    user_id VARCHAR(36) NOT NULL,
                    FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE,
                    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
                )",
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_table(
                Table::create()
                    .table(PermissionUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermissionUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(PermissionUser::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PermissionUser::UserId).uuid().not_null())
                    .take(),
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(PermissionUser::Table, PermissionUser::PermissionId)
                    .to(Permission::Table, Permission::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .take(),
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(PermissionUser::Table, PermissionUser::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PermissionUser::Table).take())
            .await
    }
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum PermissionUser {
    #[sea_orm(iden = "permission_user")]
    Table,
    Id,
    PermissionId,
    UserId,
}
