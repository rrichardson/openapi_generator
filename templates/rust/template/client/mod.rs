#![allow(clippy::ptr_arg)]
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
        Some(&self.0)
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
        {{~#if requestBody~}}
            {{~#with requestBody.content.[application/json]}}body: &{{snakecase ../operationId}}::Body,{{~/with}}
            {{~#with requestBody.content.[multipart/form-data]}}form: reqwest::multipart::Form,{{~/with}}
        {{/if~}}
    ) -> Result<{{snakecase operationId}}::Success, {{snakecase operationId}}::Error> {
        use {{snakecase ../operationId}}::*;
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
                {{~#with requestBody.content.[application/json]}}.json(&body){{~/with}}
                {{~#with requestBody.content.[multipart/form-data]}}.multipart(form){{~/with}}
            {{~/if}}
            .send().await.map_err(ReqwestError)?;
        match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {
                    Ok(Success::{{camelcase "Status" @key}}(()))
                }
                {{~else~}}
                "{{@key}}" => {
                    {{~#if content}}
                        {{~#with content.[image/png]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[image/jpeg]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[text/plain]}}let response_body = response.text().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[application/json]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                    {{~else~}}
                        let response_body = ();
                    {{~/if}}
                    {{~#if (is_http_code_success @key)}}
                    Ok(Success::{{camelcase "Status" @key}}(response_body))
                    {{else}}
                    Err(Error::{{camelcase "Status" @key}}(response_body))
                    {{~/if}}
                }
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => Err(Error::Unknown(response)),
        }
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

{{~#*inline "shortcut_to_data_model"}}

pub mod {{snakecase operationId}} {
    use super::ReqwestError;
    use displaydoc::Display;
    pub use crate::models::{{snakecase operationId}}::*;

    #[derive(Debug, thiserror::Error, Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        {{~#each responses}}
        {{~#if (not (eq @key "default"))}}
        /// Status {{@key}} error: {0:?}
        {{camelcase "Status" @key}}({{camelcase "Status" @key}}),
        {{~/if}}
        {{~/each}}
        /// Unknown: {0:?}
        Unknown(reqwest::Response),
    }
}
{{~/inline}}

{{~#each paths}}
{{~#with get}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with head}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with post}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with put}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with delete}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with options}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with trace}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with patch}}{{~> shortcut_to_data_model}}{{~/with}}
{{~/each}}