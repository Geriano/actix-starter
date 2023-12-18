use sea_orm_migration::prelude::*;

#[allow(unused_imports)]
use crate::m20230902_024725_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS tokens (
                    id VARCHAR(36) NOT NULL PRIMARY KEY,
                    user_id VARCHAR(36) NOT NULL,
                    expired_at TIMESTAMP NULL DEFAULT NULL,
                    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
                )",
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_table(
                Table::create()
                    .table(Token::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(Token::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Token::ExpiredAt)
                            .timestamp()
                            .null()
                            .extra("default null"),
                    )
                    .take(),
            )
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Token::Table, Token::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Token::Table)
                    .name("idx_tokens_user_id")
                    .col(Token::UserId)
                    .take(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Token::Table)
                    .name("idx_tokens_expired_at")
                    .col(Token::ExpiredAt)
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Token::Table).take())
            .await
    }
}

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Token {
    #[sea_orm(iden = "tokens")]
    Table,
    Id,
    UserId,
    ExpiredAt,
}
