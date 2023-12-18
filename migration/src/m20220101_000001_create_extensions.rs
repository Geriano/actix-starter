use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared("PRAGMA foreign_keys = ON")
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        #[cfg(feature = "sqlite")]
        manager
            .get_connection()
            .execute_unprepared("PRAGMA foreign_keys = OFF")
            .await?;

        #[cfg(feature = "postgres")]
        manager
            .get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }
}
