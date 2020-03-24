#![allow(clippy::ptr_arg)]
use crate::client::{ {{camelcase info.title}}, Response, Error};
use crate::models::*;
use url::{Url};
use std::sync::Arc;
use std::time::Duration;

/* Reqwest's errors are bad-mannered and recurse on their source when displayed.
 * This behavior doesn't interact well with thiserror which also recurse on error's cause
 * when displayed. To prevent this issue, this wrapper hides the error's source from thiserror.
 */
pub struct ReqwestError(pub reqwest::Error);

impl std::error::Error for ReqwestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<reqwest::Error> for ReqwestError {
    fn from(err: reqwest::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Debug for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone)]
pub struct {{camelcase info.title "Client"}} {
    pub url: Url,
    pub client: reqwest::Client,
}

{{~#*inline "async_operation_fn"}}

    pub async fn {{snakecase operationId}}(
        &self,
        {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<reqwest::Response>, Error> {
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
        let response = self.client
            .{{operation_verb}}(url)
            {{#if (has parameters "in" "query")~}}
            .query(&parameters.query())
            {{~/if}}
            {{~#if requestBody}}
            .json(&body)
            {{~/if}}
            .send().await.map_err(ReqwestError)?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                    );
                    {{camelcase "Response" @key}}(())
                }
                {{~else~}}
                "{{@key}}" => {
                    let response_body = response.json().await.map_err(ReqwestError)?;
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}
response ({{@key}}):{:?}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                        , response_body
                    );
                    {{camelcase "Response" @key}}(response_body)
                }
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => {
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                    );
                    Unspecified(response)
                },
        })
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).expect("cannot parse url");
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = reqwest::Client::builder().timeout(timeout).build().expect("bad client build");
        self
    }

    {{~#each paths}}
        {{~#with get}}{{~> async_operation_fn operation_verb="get"}}{{~/with}}
        {{~#with head}}{{~> async_operation_fn operation_verb="head"}}{{~/with}}
        {{~#with post}}{{~> async_operation_fn operation_verb="post"}}{{~/with}}
        {{~#with put}}{{~> async_operation_fn operation_verb="put"}}{{~/with}}
        {{~#with delete}}{{~> async_operation_fn operation_verb="delete"}}{{~/with}}
        {{~#with options}}{{~> async_operation_fn operation_verb="options"}}{{~/with}}
        {{~#with trace}}{{~> async_operation_fn operation_verb="trace"}}{{~/with}}
        {{~#with patch}}{{~> async_operation_fn operation_verb="patch"}}{{~/with}}
    {{~/each}}
}

// blocking

pub mod blocking {
    use crate::client::{ {{camelcase info.title}}, Response, Error};
    use crate::models::*;
    use url::{Url};
    use std::sync::Arc;
    use std::time::Duration;
    use super::ReqwestError;

    #[derive(Clone)]
    pub struct {{camelcase info.title "Client"}} {
        pub url: Url,
        pub client: reqwest::blocking::Client,
    }

    {{~#*inline "operation_fn"}}

        fn {{snakecase operationId}}(
            &self,
            {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
            {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
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
            let response = self.client
                .{{operation_verb}}(url)
                {{#if (has parameters "in" "query")~}}
                .query(&parameters.query())
                {{~/if}}
                {{~#if requestBody}}
                .json(&body)
                {{~/if}}
                .send().map_err(ReqwestError)?;
            use {{snakecase operationId}}::Response::*;
            Ok(
                match response.status().as_str() {
                {{~#each responses}}
                {{~#if (not (eq @key "default"))}}
                    {{~#if (eq @key "204")}}
                    "{{@key}}" => {
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                        );
                        {{camelcase "Response" @key}}(())
                    }
                    {{~else~}}
                    "{{@key}}" => {
                        let response_body = response.json().map_err(ReqwestError)?;
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}
    response ({{@key}}):{:?}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                            , response_body
                        );
                        {{camelcase "Response" @key}}(response_body)
                    }
                    {{~/if}}
                {{~/if}}
                {{~/each}}
                    _ => {
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                        );
                        Unspecified(response)
                    },
            })
        }
    {{~/inline}}

    impl {{camelcase info.title "Client"}} {
        pub fn new(url: &str) -> Self {
            let url = Url::parse(url).expect("cannot parse url");
            Self {
                url,
                client: reqwest::blocking::Client::new(),
            }
        }

        pub fn with_timeout(mut self, timeout: Duration) -> Self {
            self.client = reqwest::blocking::Client::builder().timeout(timeout).build().expect("bad client build");
            self
        }
    }

    impl {{camelcase info.title}} for {{camelcase info.title "Client"}} {
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
}

