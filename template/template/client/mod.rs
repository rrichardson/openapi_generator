pub mod blocking;

#[cfg(test)]
mod tests;

use crate::models::*;

pub struct {{camelcase info.title "Client"}} {
    uri: String,
}

impl {{camelcase info.title "Client"}} {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.to_string() }
    }
    {{~#each paths}}
    {{#with get}}
    pub async fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        let uri = format!("{uri}{{@../key}}", uri = self.uri
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}}
        );
        dbg!(uri.clone());
        let mut response = surf::get(uri).set_query(&parameters.query())?.await?;
        println!("{:?}", response);
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
            {{~/if}}
            {{~/each}}
                _ => Unspecified,
        })
    }
    {{~/with}}
    {{#with post}}
    pub async fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        let uri = format!("{uri}{{@../key}}", uri = self.uri
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}}
        );
        let mut response = surf::post(uri).set_query(&parameters.query())?.body_json(&parameters.body())?.await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
            {{~/if}}
            {{~/each}}
                _ => unimplemented!(),
        })
    }
    {{~/with}}
    {{#with put}}
    pub async fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        let uri = format!("{uri}{{@../key}}", uri = self.uri
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}}
        );
        let mut response = surf::put(uri).set_query(&parameters.query())?.body_json(&parameters.body())?.await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
            {{~/if}}
            {{~/each}}
                _ => unimplemented!(),
        })
    }
    {{~/with}}
    {{#with delete}}
    pub async fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        let uri = format!("{uri}{{@../key}}", uri = self.uri
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}}
        );
        let mut response = surf::delete(uri).set_query(&parameters.query())?.await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                "{{@key}}" => {{camelcase "Response" @key}}(response.body_json().await?),
            {{~/if}}
            {{~/each}}
                _ => unimplemented!(),
        })
    }
    {{~/with}}
    {{~/each}}
}
