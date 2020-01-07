use mockito::{mock, Matcher};
use crate::models::*;
use serde_json::json;

{{~#*inline "mock"}}
{{~#each responses}}
{{~#if (or (eq @key "200") (eq @key "201"))}}
pub fn mock_{{snakecase ../operationId}} (
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
    mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(url))
        .match_query(Matcher::Any)
        .with_status({{@key}})
        .with_body(json!(response_body).to_string())
        .with_header("content-type", "application/json")
}
{{~/if}}
{{~#if (eq @key "204")}}
pub fn mock_{{snakecase ../operationId}} (
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
    mock("{{shoutysnakecase ../operation_verb}}", Matcher::Exact(url))
        .match_query(Matcher::Any)
        .with_status({{@key}})
        .with_header("content-type", "application/json")
}
{{~/if}}
{{~/each}}
{{~/inline}}

{{#each paths}}
    {{~#with get}}{{~> mock path=@../key title=../../info.title operation_verb="get" noBody=true}}{{~/with}}
    {{~#with post}}{{~> mock path=@../key title=../../info.title operation_verb="post"}}{{~/with}}
    {{~#with put}}{{~> mock path=@../key title=../../info.title operation_verb="put"}}{{~/with}}
    {{~#with delete}}{{~> mock path=@../key title=../../info.title operation_verb="delete"}}{{~/with}}
{{/each}}