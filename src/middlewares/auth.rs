use core::future::Future;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;

use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use sea_orm::prelude::*;
use uuid::Uuid;

use crate::common::{base58, time};
use crate::models::{permission_user, permissions, role_user, roles, tokens, users, Id};
use crate::responses::Unauthorized;

const CACHE: u64 = 1000 * 60 * 5;

#[derive(Clone)]
pub struct Auth {
    pub user: users::Model,
    pub permissions: Vec<permissions::Model>,
    pub roles: Vec<roles::Model>,
}

impl Auth {
    pub async fn authenticate(
        db: Data<DatabaseConnection>,
        cache: Data<Authenticated>,
        header: Option<HeaderValue>,
    ) -> Result<Self, Unauthorized> {
        if header.is_none() {
            return Err(Unauthorized {
                message: "Token not found".to_string(),
            });
        }

        let header = header.unwrap();
        let header = header.to_str().unwrap();
        let id = Auth::parse(header)?;

        if let Some((_, auth)) = cache.clear().get(&id) {
            return Ok(auth);
        }

        let db: &DatabaseConnection = &db;
        let token = tokens::Entity::find()
            .find_also_related(users::Entity)
            .filter(tokens::Column::Id.eq(id.clone()))
            .one(db)
            .await;

        if let Err(e) = token {
            return Err(Unauthorized {
                message: e.to_string(),
            });
        }

        let token = token.unwrap();

        if token.is_none() {
            return Err(Unauthorized {
                message: "Token not found".to_string(),
            });
        }

        let user = token.unwrap().1.unwrap();
        let permissions = permissions::Entity::find()
            .find_with_related(permission_user::Entity)
            .filter(permission_user::Column::UserId.eq(user.id.clone()))
            .all(db)
            .await;

        if let Err(e) = permissions {
            return Err(Unauthorized {
                message: e.to_string(),
            });
        }

        let roles = roles::Entity::find()
            .find_with_related(role_user::Entity)
            .filter(role_user::Column::UserId.eq(user.id.clone()))
            .all(db)
            .await;

        if let Err(e) = roles {
            return Err(Unauthorized {
                message: e.to_string(),
            });
        }

        let (permissions, roles) = (permissions.unwrap(), roles.unwrap());

        Ok(cache.set(
            id,
            time::unix() + CACHE,
            Auth {
                user,
                permissions: permissions
                    .iter()
                    .map(|(permission, _)| permission.clone())
                    .collect(),
                roles: roles.iter().map(|(role, _)| role.clone()).collect(),
            },
        ))
    }

    fn parse<T: ToString>(token: T) -> Result<Id, Unauthorized> {
        let token = token.to_string();
        let token = token.split(" ").collect::<Vec<&str>>();

        if token.len() != 2 {
            return Err(Unauthorized {
                message: "Invalid token".to_string(),
            });
        }

        if token[0].trim().to_lowercase() != "bearer" {
            return Err(Unauthorized {
                message: "Invalid token type".to_string(),
            });
        }

        let token = token[1];
        let token = base58::decode(token).map_err(|e| Unauthorized {
            message: e.to_string(),
        })?;

        #[cfg(feature = "sqlite")]
        {
            use std::str::FromStr;

            let string = String::from_utf8_lossy(&token);
            let uuid = Uuid::from_str(&string).map_err(|e| Unauthorized {
                message: e.to_string(),
            })?;

            Ok(uuid.into())
        }

        #[cfg(feature = "postgres")]
        {
            let uuid = Uuid::from_slice(&token).map_err(|e| Unauthorized {
                message: e.to_string(),
            })?;

            Ok(uuid.into())
        }
    }
}

pub struct Authenticated(pub Mutex<HashMap<Id, (u64, Auth)>>);

impl Authenticated {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn all(&self) -> HashMap<Id, (u64, Auth)> {
        self.0.lock().unwrap().clone()
    }

    pub fn get(&self, id: &Id) -> Option<(u64, Auth)> {
        self.0.lock().unwrap().get(id).cloned()
    }

    pub fn set(&self, id: Id, expired: u64, auth: Auth) -> Auth {
        self.0.lock().unwrap().insert(id, (expired, auth.clone()));

        auth
    }

    pub fn remove(&self, id: &Id) {
        self.0.lock().unwrap().remove(id);
    }

    pub fn clear(&self) -> &Self {
        for (id, (expired, _)) in self.all() {
            if time::unix() > expired {
                self.remove(&id);
            }
        }

        self
    }
}

impl FromRequest for Auth {
    type Error = Unauthorized;
    type Future = Pin<Box<dyn Future<Output = Result<Auth, Unauthorized>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let db = req.app_data::<Data<DatabaseConnection>>().cloned().unwrap();
        let cache = req.app_data::<Data<Authenticated>>().cloned().unwrap();
        let authorization = req.headers().get("Authorization").cloned();

        Box::pin(Auth::authenticate(db, cache, authorization))
    }
}
