use actix_web::{web::*, App, HttpServer, Responder, HttpResponse};
use std::error::Error;
use crate::models::*;

pub trait {{camelcase info.title}} {
    type Error: std::error::Error;

{{#each paths}}
    {{~#with get}}
    fn {{snakecase operationId}}(&self, _parameters: {{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, Self::Error> {
        unimplemented!()
    }
    {{~/with}}
{{/each}}
}

{{#each paths}}
    {{~#with get}}
    {{#if summary}}/// {{summary}}{{/if}}
    {{~#if description}}/// {{description}}{{/if}}
    fn {{snakecase operationId}}<Server: {{camelcase ../../info.title}}>(
        server: Data<Server>,{{!-- {{~#if parameters}} --}}
        query: Query<{{snakecase operationId}}::Query>,
        path: Path<{{snakecase operationId}}::Path>,
    {{!-- {{~/if}} --}}
    ) -> impl Responder {
        use {{snakecase operationId}}::*;
        let parameters = Parameters::new(query.into_inner(), path.into_inner());
        match server.{{snakecase operationId}}(parameters) {
            Ok(response) => HttpResponse::Ok().body("{{summary}}"),
            Err(err) => HttpResponse::InternalServerError().body(err.description().to_string()),
        }
    }
    {{/with}}
{{~/each}}

pub fn run<Server: {{camelcase info.title}} + Send + Sync + Clone + 'static>(
    server: Server,
) -> std::io::Result<()> {
    HttpServer::new(move ||
        App::new()
            .data(server.clone())
            {{~#each paths}}
                .service(
                    resource("{{@key}}")
                        {{~#with get}}
                        .route(get().to({{snakecase operationId}}::<Server>))
                        {{~/with}}
                )
            {{~/each}}
    )
    .bind("127.0.0.1:8080")?
    .run()
}
