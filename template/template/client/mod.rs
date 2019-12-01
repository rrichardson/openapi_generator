pub mod blocking;

#[cfg(test)]
mod tests;

use crate::models::*;

pub struct {{camelcase info.title "Client"}} {
    uri: String,
}

{{~#*inline "operation_fn"}}

    pub async fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response<surf::Response>, surf::Exception> {
        let uri = format!("{uri}{{@../key}}", uri = self.uri
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}}
        );
        let mut response = surf::{{operation_verb}}(uri)
            {{~#if parameters}}
            .set_query(&parameters.query())?
            {{~/if}}
            .await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {{camelcase "Response" @key}}(response),
                {{~else~}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => Unspecified(response),
        })
    }
{{~/inline~}}

impl {{camelcase info.title "Client"}} {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
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
