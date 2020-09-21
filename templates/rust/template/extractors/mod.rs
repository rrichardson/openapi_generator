use actix_web::error::ErrorBadRequest;
use actix_web::{dev, App, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Authorization {
    pub authorization: String,
}

impl FromRequest for Authorization {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let headers = req.headers();
        match headers.get("authorization") {
            Some(authorization) => match authorization.to_str() {
                Ok(authorization) => ok(Authorization {
                    authorization: authorization.to_string(),
                }),
                Err(_) => err(ErrorBadRequest("authorization should be a string")),
            },
            None => err(ErrorBadRequest("missing authorization header")),
        }
    }
}
