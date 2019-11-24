use crate::models::*;
use actix_web::{web::*, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

pub trait SwaggerPetstore {
    type Error: std::error::Error;

    fn list_pets(
        &self,
        parameters: list_pets::Parameters,
    ) -> Result<list_pets::Response, Self::Error> {
        unimplemented!()
    }

    fn show_pet_by_id(
        &self,
        parameters: show_pet_by_id::Parameters,
    ) -> Result<show_pet_by_id::Response, Self::Error> {
        unimplemented!()
    }
}

/// List all pets
fn list_pets<Server: SwaggerPetstore>(
    server: Data<Server>,
    query: Query<list_pets::Query>,
    path: Path<list_pets::Path>,
) -> impl Responder {
    use list_pets::*;
    let parameters = Parameters::new(query.into_inner(), path.into_inner());
    match server.list_pets(parameters) {
        Ok(response) => HttpResponse::Ok().body("List all pets"),
        Err(err) => HttpResponse::InternalServerError().body(err.description().to_string()),
    }
}

/// Info for a specific pet
fn show_pet_by_id<Server: SwaggerPetstore>(
    server: Data<Server>,
    query: Query<show_pet_by_id::Query>,
    path: Path<show_pet_by_id::Path>,
) -> impl Responder {
    use show_pet_by_id::*;
    let parameters = Parameters::new(query.into_inner(), path.into_inner());
    match server.show_pet_by_id(parameters) {
        Ok(response) => HttpResponse::Ok().body("Info for a specific pet"),
        Err(err) => HttpResponse::InternalServerError().body(err.description().to_string()),
    }
}

pub fn run<Server: SwaggerPetstore + Send + Sync + Clone + 'static>(
    server: Server,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(resource("/pets").route(get().to(list_pets::<Server>)))
            .service(resource("/pets/{petId}").route(get().to(show_pet_by_id::<Server>)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
