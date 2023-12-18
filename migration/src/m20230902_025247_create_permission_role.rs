use sea_orm_migration::prelude::*;

#[allow(unused_imports)]
use crate::{m20230902_024928_create_permissions::Permission, m20230902_025106_create_roles::Role};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS permission_role (
                    id VARCHAR(36) NOT NULL PRIMARY KEY,
                    permission_id VARCHAR(36) NOT NULL,
                    role_id VARCHAR(36) NOT NULL,
                    FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE,
                    FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
                )",
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_table(
                Table::create()
                    .table(PermissionRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermissionRole::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(PermissionRole::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PermissionRole::RoleId).uuid().not_null())
                    .take(),
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(PermissionRole::Table, PermissionRole::PermissionId)
                    .to(Permission::Table, Permission::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .take(),
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(PermissionRole::Table, PermissionRole::RoleId)
                    .to(Role::Table, Role::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PermissionRole::Table).take())
            .await
    }
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum PermissionRole {
    #[sea_orm(iden = "permission_role")]
    Table,
    Id,
    PermissionId,
    RoleId,
}
