#![allow(clippy::ptr_arg)]

#[cfg(feature = "surf-client")]
pub mod surf;

#[cfg(feature = "reqwest-client")]
pub mod reqwest;

#[cfg(all(test, feature = "example"))]
mod tests;

use crate::models::*;
use mockiato::mockable;
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;
use displaydoc::Display;
#[cfg(feature = "surf-client")]
pub use self::surf::{{camelcase info.title "Client"}};

pub type Response = String;

#[derive(Debug, thiserror::Error, Display)]
pub enum Error {
    /// Request failed
    SurfClient(#[from] ::surf::Exception),
    /// Request failed
    #[cfg(feature = "reqwest-client")]
    ReqwestClient(#[from] ::reqwest::Error),
    /// IO error occured while retrieving response body
    Io(#[from] std::io::Error),
    /// Request body serialization to JSON failed
    BodySerialization(#[from] serde_json::Error),
    /// Request parameters serialization failed
    ParametersSerialization(#[from] serde_urlencoded::ser::Error),
    /// Timeout occured during request
    Timeout(#[from] async_std::future::TimeoutError),
}

{{~#*inline "trait_operation_fn"}}
    fn {{snakecase operationId}}(
        &self,
        {{~#if parameters}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<Response>, Error> {
        unimplemented!("{{snakecase operationId}}")
    }
{{/inline}}

#[mockable]
pub trait {{camelcase info.title}} {
    {{#each paths}}
        {{~#with get}}{{~> trait_operation_fn operation_verb="get"}}{{~/with}}
        {{~#with head}}{{~> trait_operation_fn operation_verb="head"}}{{~/with}}
        {{~#with post}}{{~> trait_operation_fn operation_verb="post"}}{{~/with}}
        {{~#with put}}{{~> trait_operation_fn operation_verb="put"}}{{~/with}}
        {{~#with delete}}{{~> trait_operation_fn operation_verb="delete"}}{{~/with}}
        {{~#with options}}{{~> trait_operation_fn operation_verb="options"}}{{~/with}}
        {{~#with trace}}{{~> trait_operation_fn operation_verb="trace"}}{{~/with}}
        {{~#with patch}}{{~> trait_operation_fn operation_verb="patch"}}{{~/with}}
    {{~/each}}
}

#[cfg(feature = "surf-client")]
pub mod blocking {
    pub use super::surf::blocking::{{camelcase info.title "Client"}};
}