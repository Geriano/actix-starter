use std::usize::MAX;

use actix_cors::Cors;
use actix_web::web::{
    self, Data, FormConfig, JsonConfig, PathConfig, PayloadConfig, ServiceConfig,
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::api::Doc;
use crate::middlewares::auth::Authenticated;
use crate::responses::BadRequest;
use crate::route;

pub fn configure(
    db: DatabaseConnection,
) -> impl Fn(&mut ServiceConfig) + Clone + Send + Sync + 'static {
    move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .max_age(3600);

        route::route(cfg)
            .app_data(cors)
            .app_data(PayloadConfig::new(MAX))
            .app_data(PathConfig::default().error_handler(|e, _| {
                BadRequest {
                    message: e.to_string(),
                }
                .into()
            }))
            .app_data(JsonConfig::default().limit(MAX).error_handler(|e, _| {
                BadRequest {
                    message: e.to_string(),
                }
                .into()
            }))
            .app_data(FormConfig::default().limit(MAX).error_handler(|e, _| {
                BadRequest {
                    message: e.to_string(),
                }
                .into()
            }))
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(Authenticated::new()))
            .service(web::redirect("/", "/doc"))
            .service(web::redirect("/doc", "/doc/"))
            .service(SwaggerUi::new("/doc/{_:.*}").urls(vec![(
                Url::new("learning-management-system", "/doc/api.json"),
                Doc::openapi(),
            )]));
    }
}
