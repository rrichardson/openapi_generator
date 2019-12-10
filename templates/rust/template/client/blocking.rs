use crate::models::*;
use async_std::task::block_on;
use url::{Url};

#[derive(Clone)]
pub struct {{camelcase info.title "Client"}} {
    client: super::{{camelcase info.title "Client"}},
}

{{~#*inline "operation_fn"}}

    pub fn {{snakecase operationId}}(&self, {{~#if (not (or parameters requestBody))~}}_{{~/if}}parameters: {{~#if (or parameters requestBody)~}}&{{~/if}}{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response<surf::Response>, surf::Exception> {
        block_on(self.client.{{snakecase operationId}}({{~#if (or parameters requestBody)~}}parameters{{~else~}}(){{~/if}}))
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
    pub fn new(url: &str) -> Self {
        Self { client: super::{{camelcase info.title "Client"}}::new(url) }
    }

    pub fn url(&self) -> Url {
        self.client.url.clone()
    }

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
