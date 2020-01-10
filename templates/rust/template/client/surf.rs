#![allow(clippy::ptr_arg)]

use crate::models::*;
use url::Url;
use super::{Response, Error};

#[derive(Clone, Debug)]
pub struct {{camelcase info.title "Client"}} {
    pub url: Url,
}

{{~#*inline "operation_fn"}}

    pub async fn {{snakecase operationId}}(
        &self,
        {{~#if parameters}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<Response>, Error> {
        let url = self.url.join(
            {{#if (has parameters "in" "path")~}}
            format!("{{@../key}}"
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}})
            {{~else~}}
            "{{@../key}}"
            {{~/if~}}
            .trim_start_matches('/')
        ).expect("url parse error");
        let mut response = surf::{{operation_verb}}(url)
            {{~#if (has parameters "in" "query")}}
            .set_query(&parameters.query())?
            {{~/if}}
            {{~#if requestBody}}
            .body_json(body)?
            {{~/if}}
            .await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {{camelcase "Response" @key}}(()),
                {{~else~}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => Unspecified(format!("{:?}", response)),
        })
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).expect("cannot parse url");
        Self { url }
    }

    {{~#each paths}}
        {{~#with get}}{{~> operation_fn operation_verb="get"}}{{~/with}}
        {{~#with head}}{{~> operation_fn operation_verb="head"}}{{~/with}}
        {{~#with post}}{{~> operation_fn operation_verb="post"}}{{~/with}}
        {{~#with put}}{{~> operation_fn operation_verb="put"}}{{~/with}}
        {{~#with delete}}{{~> operation_fn operation_verb="delete"}}{{~/with}}
        {{~#with options}}{{~> operation_fn operation_verb="options"}}{{~/with}}
        {{~#with trace}}{{~> operation_fn operation_verb="trace"}}{{~/with}}
        {{~#with patch}}{{~> operation_fn operation_verb="patch"}}{{~/with}}
    {{~/each}}
}

pub mod blocking {
    use crate::models::*;
    use async_std::{future::timeout, task::block_on};
    use url::{Url};
    use super::super::{ {{camelcase info.title}}, Response, Error};
    use std::time::Duration;

    #[derive(Clone)]
    pub struct {{camelcase info.title "Client"}} {
        client: super::{{camelcase info.title "Client"}},
        timeout: Option<Duration>,
    }

    {{~#*inline "blocking_operation_fn"}}

    fn {{snakecase operationId}}(
        &self,
        {{~#if parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<Response>, Error> {
        let future_result = self.client.{{snakecase operationId}}(
            {{~#if parameters}}parameters,{{/if}}
            {{~#if requestBody}}body,{{/if}}
        );
        let result = if let Some(duration) = self.timeout {
            block_on(timeout(duration, future_result))?
        } else {
            block_on(future_result)
        }?;
        Ok(result)
    }
    {{~/inline}}

    impl {{camelcase info.title "Client"}} {
        pub fn new(url: &str) -> Self {
            Self {
                client: super::{{camelcase info.title "Client"}}::new(url),
                timeout: None,
            }
        }

        pub fn with_timeout(mut self, timeout: Duration) -> Self {
            self.timeout = Some(timeout);
            self
        }

        pub fn url(&self) -> Url {
            self.client.url.clone()
        }
    }

    impl {{camelcase info.title}} for {{camelcase info.title "Client"}} {
        {{~#each paths}}
            {{~#with get}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with head}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with post}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with put}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with delete}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with options}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with trace}}{{~> blocking_operation_fn}}{{~/with}}
            {{~#with patch}}{{~> blocking_operation_fn}}{{~/with}}
        {{~/each}}
    }
}