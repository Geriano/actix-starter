use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};

use actix_web::http::StatusCode;
use actix_web::{body::BoxBody, HttpRequest};
use actix_web::{HttpResponse, Responder, ResponseError};
use serde::Serialize;
use utoipa::{IntoResponses, ToSchema};

use crate::models::Id;

macro_rules! response {
    ($name:tt, $code:tt, $status:tt, $body:tt) => {
        #[derive(Serialize, ToSchema, IntoResponses)]
        #[response(status = $code)]
        pub struct $name $body

        impl $name {
            pub fn response(&self) -> HttpResponse {
                HttpResponse::$status().json(self)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", serde_json::to_string(self).unwrap())
            }
        }

        impl Into<HttpResponse> for $name {
            fn into(self) -> HttpResponse {
                self.response()
            }
        }

        impl ResponseError for $name {
            fn status_code(&self) -> StatusCode {
                self.response().status()
            }

            fn error_response(&self) -> HttpResponse {
                self.response()
            }
        }

        impl Responder for $name {
            type Body = BoxBody;

            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                self.response()
            }
        }
    };
}

response!(Ok, 200, Ok, {
    #[schema()]
    pub message: String,
});

response!(Created, 201, Created, {
    #[schema()]
    pub message: String,
});

response!(CreatedWithId, 201, Created, {
    #[schema()]
    pub id: Id,
    #[schema()]
    pub message: String,
});

response!(BadRequest, 400, BadRequest, {
    #[schema()]
    pub message: String,
});

response!(Unauthorized, 401, Unauthorized, {
    #[schema()]
    pub message: String,
});

response!(Forbidden, 403, Forbidden, {
    #[schema()]
    pub message: String,
});

response!(NotFound, 404, NotFound, {
    #[schema()]
    pub message: String,
});

response!(Conflict, 409, Conflict, {
    #[schema()]
    pub message: String,
});

response!(UnprocessableEntity, 422, UnprocessableEntity, {
    #[schema()]
    pub errors: HashMap<&'static str, Vec<&'static str>>,
});

response!(InternalServerError, 500, InternalServerError, {
    #[schema()]
    pub message: String,
});
