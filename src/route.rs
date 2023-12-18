use actix_web::web::ServiceConfig;

use crate::controllers;

pub fn route(app: &mut ServiceConfig) -> &mut ServiceConfig {
    app.service(controllers::auth::login)
        .service(controllers::auth::authenticate)
        .service(controllers::auth::logout)
        // user
        .service(controllers::user::paginate)
        .service(controllers::user::store)
        .service(controllers::user::show)
        .service(controllers::user::update_general_information)
        .service(controllers::user::update_password)
        .service(controllers::user::delete)
        // permission
        .service(controllers::permission::paginate)
        .service(controllers::permission::store)
        .service(controllers::permission::show)
        .service(controllers::permission::update)
        .service(controllers::permission::delete)
        // role
        .service(controllers::role::paginate)
        .service(controllers::role::store)
        .service(controllers::role::show)
        .service(controllers::role::update)
        .service(controllers::role::delete)
}
