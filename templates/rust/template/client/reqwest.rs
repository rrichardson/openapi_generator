#![allow(clippy::ptr_arg)]

pub mod blocking {
    use crate::client::{ {{camelcase info.title}}, Response, Error};
    use crate::models::*;
    use url::{Url};
    use std::sync::Arc;

    #[derive(Clone)]
    pub struct {{camelcase info.title "Client"}} {
        pub url: Url,
        pub client: reqwest::blocking::Client,
    }

    {{~#*inline "operation_fn"}}

        fn {{snakecase operationId}}(
            &self,
            {{~#if parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
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
                {{~#if requestBody}}
                .json(&body)
                {{~/if}}
                .send()?;
            use {{snakecase operationId}}::Response::*;
            Ok(
                match response.status().as_str() {
                {{~#each responses}}
                {{~#if (not (eq @key "default"))}}
                    {{~#if (eq @key "204")}}
                    "{{@key}}" => {{camelcase "Response" @key}}(()),
                    {{~else~}}
                    "{{@key}}" => {{camelcase "Response" @key}}(response.json()?),
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
            Self {
                url,
                client: reqwest::blocking::Client::new(),
            }
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

