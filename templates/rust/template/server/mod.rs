#![allow(clippy::unit_arg, clippy::redundant_clone)]

use actix_web::{web::*, Responder, HttpResponse, dev::HttpResponseBuilder, http::StatusCode};
use std::error::Error;
use crate::models::*;

{{~#*inline "operation_fn_trait"}}

    fn {{snakecase operationId}}(
        &self,
        _parameters: {{snakecase operationId}}::Parameters,
        {{#unless noBody~}} _body: {{snakecase operationId}}::Body, {{~/unless}}
    ) -> Result<{{snakecase operationId}}::Response<HttpResponse>, Self::Error> {
        unimplemented!()
    }
{{~/inline}}

pub trait {{camelcase info.title}} {
    type Error: std::error::Error;
{{~#each paths}}
    {{~#with get}}{{~> operation_fn_trait noBody=true}}{{~/with}}
    {{~#with head}}{{~> operation_fn_trait noBody=true}}{{~/with}}
    {{~#with post}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with put}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with delete}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with options}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with trace}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with patch}}{{~> operation_fn_trait}}{{~/with}}
{{~/each}}
}
{{#*inline "operation_fn"}}
{{#if summary}}/// {{summary}}{{/if}}
{{~#if description}}/// {{description}}{{/if}}
async fn {{snakecase operationId}}<Server: {{camelcase title}}>(
    server: Data<Server>,{{!-- {{~#if parameters}} --}}
    {{~#if (has parameters "in" "query")~}}
    query: Query<{{snakecase operationId}}::Query>,
    {{~/if}}
    {{~#if (has parameters "in" "path")~}}
    path: Path<{{snakecase operationId}}::Path>,
    {{~/if}}
    {{~#if (and requestBody (not noBody))}}
    body: Json<{{snakecase operationId}}::Body>,
    {{~/if}}
) -> impl Responder {
    use {{snakecase operationId}}::*;
    let parameters = Parameters::new(
        {{~#if (has parameters "in" "query")~}}query.into_inner(),{{~/if}}
        {{~#if (has parameters "in" "path")~}}path.into_inner(),{{~/if}}
    );
    {{~#unless noBody}}
    let body =
        {{~#if requestBody}}
            body.into_inner();
        {{~else~}}
            {{snakecase operationId}}::Body {};
         {{~/if}}
    {{~/unless}}
    match server.{{snakecase operationId}}(parameters {{~#unless noBody}}, body{{/unless}}) {
        {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
        Ok(Response::{{camelcase "Response" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(response),
            {{~/if}}
        {{~/each}}
        Ok(Response::Unspecified(response)) => response,
        Err(err) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
{{~/inline}}

fn err_to_string(err: &dyn std::error::Error) -> String {
    let mut errors_str = Vec::new();
    let mut current_err = err.source();
    while let Some(err) = current_err {
        errors_str.push(err.to_string());
        current_err = err.source();
    }
    format!("error: {}\n\ncaused by:\n\t{}", err, errors_str.as_slice().join("\n\t"))
}

{{#each paths}}
    {{~#with get}}{{~> operation_fn title=../../info.title noBody=true}}{{~/with}}
    {{~#with head}}{{~> operation_fn title=../../info.title noBody=true}}{{~/with}}
    {{~#with post}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with put}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with delete}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with options}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with trace}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with patch}}{{~> operation_fn title=../../info.title}}{{~/with}}
{{~/each}}

pub fn config<Server: {{camelcase info.title}} + Send + Sync + Clone + 'static>(
    app: &mut ServiceConfig,
) {
    app
    {{~#each paths}}
        .service(
            resource("{{@key}}")
                {{~#with get}}
                .route(get().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with head}}
                .route(head().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with post}}
                .route(post().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with put}}
                .route(put().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with delete}}
                .route(delete().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with options}}
                .route(options().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with trace}}
                .route(trace().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with patch}}
                .route(patch().to({{snakecase operationId}}::<Server>))
                {{~/with}}
        )
    {{~/each}};
}
