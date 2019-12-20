#![allow(clippy::ptr_arg)]

use crate::models::*;
use async_std::task::block_on;
use url::{Url};
use super::{ {{camelcase info.title}}, Response, Error};

#[derive(Clone, Debug)]
pub struct {{camelcase info.title "Client"}} {
    client: super::{{camelcase info.title "Client"}},
}

{{~#*inline "operation_fn"}}

    fn {{snakecase operationId}}(
        &self,
        {{~#if parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<Response>, Error> {
        block_on(self.client.{{snakecase operationId}}(
            {{~#if parameters}}parameters,{{/if}}
            {{~#if requestBody}}body,{{/if}}
        ))
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
    pub fn new(url: &str) -> Self {
        Self { client: super::{{camelcase info.title "Client"}}::new(url) }
    }

    pub fn url(&self) -> Url {
        self.client.url.clone()
    }
}

impl {{camelcase info.title}} for {{camelcase info.title "Client"}} {
    {{~#each paths}}
        {{~#with get}}{{~> operation_fn}}{{~/with}}
        {{~#with head}}{{~> operation_fn}}{{~/with}}
        {{~#with post}}{{~> operation_fn}}{{~/with}}
        {{~#with put}}{{~> operation_fn}}{{~/with}}
        {{~#with delete}}{{~> operation_fn}}{{~/with}}
        {{~#with options}}{{~> operation_fn}}{{~/with}}
        {{~#with trace}}{{~> operation_fn}}{{~/with}}
        {{~#with patch}}{{~> operation_fn}}{{~/with}}
    {{~/each}}
}