use mockito::{mock, Matcher};
use crate::models;
use serde_json::json;

{{~#*inline "mock"}}
pub mod {{snakecase ../operationId}} {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;

    {{~#each responses}}
    {{~#if (or (eq @key "200") (eq @key "201"))}}
    pub struct MockBuilder {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder {

        pub fn new(
            {{~#if ../parameters}} parameters: &{{snakecase ../operationId}}::Parameters,{{/if}}
        ) -> Self {
            let url =
                {{#if (has ../parameters "in" "path")~}}
                format!("{{../path}}"
                {{~#each ../parameters}}
                    {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
                {{~/each~}})
                {{~else~}}
                "{{../path}}".to_string()
                {{~/if~}};
            Self {
                counter: Arc::new(AtomicUsize::new(0)),
                responses: Vec::new(),
                url,
            }
        }

        pub fn add_response(&mut self, response_body: {{snakecase ../operationId}}::Response{{@key}},) {
            self.responses.push(json!(response_body).to_string());
        }

        pub fn create(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            mockito::mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .with_status({{@key}})
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                .expect(self.responses.len())
                .create()
        }
    }

    pub fn mock (
        {{~#if ../parameters}} parameters: &{{snakecase ../operationId}}::Parameters,{{/if}}
        response_body: {{snakecase ../operationId}}::Response{{@key}},
        ) -> mockito::Mock {
        let url =
            {{#if (has ../parameters "in" "path")~}}
            format!("{{../path}}"
            {{~#each ../parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}})
            {{~else~}}
            "{{../path}}".to_string()
            {{~/if~}};
        mockito::mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(url))
            .match_query(Matcher::Any)
            .with_status({{@key}})
            .with_body(json!(response_body).to_string())
            .with_header("content-type", "application/json")
    }
    {{~/if}}

    {{~#if (eq @key "204")}}
    pub fn mock (
        {{~#if ../parameters}} parameters: &{{snakecase ../operationId}}::Parameters,{{/if}}
        ) -> mockito::Mock {
        let url =
            {{#if (has ../parameters "in" "path")~}}
            format!("{{../path}}"
            {{~#each ../parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}})
            {{~else~}}
            "{{../path}}".to_string()
            {{~/if~}};
        mockito::mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(url))
            .match_query(Matcher::Any)
            .with_status({{@key}})
            .with_header("content-type", "application/json")
    }
    {{~/if}}
    {{~/each}}
}
{{~/inline}}

{{#each paths}}
    {{~#with get}}{{~> mock path=@../key title=../../info.title operation_verb="get" noBody=true}}{{~/with}}
    {{~#with post}}{{~> mock path=@../key title=../../info.title operation_verb="post"}}{{~/with}}
    {{~#with put}}{{~> mock path=@../key title=../../info.title operation_verb="put"}}{{~/with}}
    {{~#with delete}}{{~> mock path=@../key title=../../info.title operation_verb="delete"}}{{~/with}}
{{/each}}