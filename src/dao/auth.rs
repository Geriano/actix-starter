use sea_orm::prelude::*;

use crate::common::log;
use crate::models::{tokens, users, Id, Timestamp};

pub async fn generate(
    db: &DatabaseConnection,
    user: &users::Model,
    expired_at: Option<Timestamp>,
) -> Result<tokens::Model, DbErr> {
    let token = tokens::ActiveModel::from(tokens::Model {
        id: Uuid::new_v4().into(),
        user_id: user.id.clone(),
        expired_at,
    });

    match token.insert(db).await {
        Err(e) => {
            log::error!(generate, "{}", e);

            Err(e)
        }
        Ok(token) => Ok(token),
    }
}

pub async fn delete<I: Into<Id>>(db: &DatabaseConnection, user_id: I) -> Result<(), DbErr> {
    let id: Id = user_id.into();

    tokens::Entity::delete_many()
        .filter(tokens::Column::UserId.eq(id))
        .exec(db)
        .await?;

    Ok(())
}
