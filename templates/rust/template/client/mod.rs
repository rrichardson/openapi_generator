#![allow(clippy::ptr_arg)]

pub mod blocking;

#[cfg(all(test, feature = "example"))]
mod tests;

use crate::models::*;
use url::Url;

#[derive(Clone)]
pub struct {{camelcase info.title "Client"}} {
    pub url: Url,
}

{{~#*inline "operation_fn"}}

    pub async fn {{snakecase operationId}}(
        &self,
        {{~#if parameters}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<surf::Response>, surf::Exception> {
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
                _ => Unspecified(response),
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
