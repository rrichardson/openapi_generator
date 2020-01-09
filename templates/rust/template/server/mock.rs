#![allow(clippy::ptr_arg)]

{{~#*inline "mock"}}
pub mod {{snakecase ../operationId}} {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use mockito::Matcher;
    use serde_json::json;
    use crate::models::*;
    use std::sync::Arc;

    {{~#each responses}}
    {{~#if (or (eq @key "200") (or (eq @key "201") (eq @key "204")))}}
    pub struct MockBuilder {
        counter: Arc<AtomicUsize>,
        responses: Vec<String>,
        url: String,
    }

    impl MockBuilder {

        #[allow(clippy::new_without_default)]
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

        pub fn with_response(mut self, response_body: {{snakecase ../operationId}}::Response{{@key}},) -> Self {
            self.responses.push(json!(response_body).to_string());
            self
        }

        pub fn with_responses(mut self, response_body: {{snakecase ../operationId}}::Response{{@key}}, expect: usize) -> Self {
            self.responses.extend(std::iter::repeat(json!(response_body).to_string()).take(expect));
            self
        }

        pub fn build(&self) -> mockito::Mock {
            let counter = self.counter.clone();
            let responses = self.responses.clone();
            mockito::mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(self.url.clone()))
                .match_query(Matcher::Any)
                .with_status({{@key}})
                {{~#if (not (eq @key "204"))}}
                .with_body_from_fn(move |w| {
                    let c = counter.load(Ordering::Relaxed);
                    let response = responses.get(c).unwrap();
                    if c < responses.len() - 1 {
                        counter.store(c + 1, Ordering::Relaxed);
                    }
                    w.write_all((*response).as_bytes())
                })
                .with_header("content-type", "application/json")
                {{~/if}}
                .expect(self.responses.len())
        }

        pub fn create(&self) -> mockito::Mock {
            self.build().create()
        }
    }

    pub fn mock (
        {{~#if ../parameters}} parameters: &{{snakecase ../operationId}}::Parameters,{{/if}}
        ) -> MockBuilder {
        MockBuilder::new({{~#if ../parameters}}parameters{{/if}})
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