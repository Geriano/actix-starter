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
                "CREATE TABLE IF NOT EXISTS permissions (
                    id VARCHAR(36) NOT NULL PRIMARY KEY,
                    code VARCHAR(255) NOT NULL UNIQUE,
                    name VARCHAR(255) NOT NULL
                )",
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(
                        ColumnDef::new(Permission::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Permission::Name).string().not_null())
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Permission::Table)
                    .col(Permission::Code)
                    .name("idx_permissions_code")
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Permission::Table)
                    .col(Permission::Name)
                    .name("idx_permissions_name")
                    .take(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).take())
            .await
    }
}

#[derive(DeriveIden)]
#[allow(dead_code)]
pub enum Permission {
    #[sea_orm(iden = "permissions")]
    Table,
    Id,
    Code,
    Name,
}
