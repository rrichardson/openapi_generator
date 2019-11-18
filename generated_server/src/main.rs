#![allow(dead_code, unused_variables)]

use actix_web::{web::*, App, HttpResponse, HttpServer, Responder};
use std::error::Error;

pub mod components {
    pub mod schemas {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize)]
        pub struct Error {
            pub code: Option<i32>,
            pub message: Option<String>,
        }

        #[derive(Deserialize, Serialize)]
        pub struct Pet {
            pub id: Option<i64>,
            pub name: Option<String>,
            pub tag: Option<String>,
        }

        #[derive(Deserialize, Serialize)]
        pub struct Pets {
            pub data: Vec<crate::components::schemas::Pet>,
        }
    }
}

pub mod list_pets {
    use actix_web::HttpResponse;
    use serde::{Deserialize, Serialize};

    /// Parameters for list_pets operation
    #[derive(Deserialize)]
    pub struct Parameters {
        /// How many items to return at one time (max 100)
        pub limit: i32,
    }

    impl Parameters {
        pub fn new(query: Query, path: Path) -> Self {
            Self { limit: query.limit }
        }
    }

    /// Query parameters for list_pets operation
    #[derive(Deserialize)]
    pub struct Query {
        /// How many items to return at one time (max 100)
        pub limit: i32,
    }

    /// Path parameters for list_pets operation
    #[derive(Deserialize)]
    pub struct Path {}

    #[derive(Serialize)]
    pub enum Response {
        Response200(Response200),
        Unspecified(HttpResponse),
    }

    /// A paged array of pets
    pub type Response200 = crate::components::schemas::Pets;
}

pub mod create_pets {
    use actix_web::HttpResponse;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub enum Response {
        Response201(Response201),
        Unspecified(HttpResponse),
    }

    /// Null response
    #[derive(Deserialize, Serialize)]
    pub struct Response201;
}

pub mod show_pet_by_id {
    use actix_web::HttpResponse;
    use serde::{Deserialize, Serialize};

    /// Parameters for show_pet_by_id operation
    #[derive(Deserialize)]
    pub struct Parameters {
        /// The id of the pet to retrieve
        pub pet_id: String,
    }

    impl Parameters {
        pub fn new(query: Query, path: Path) -> Self {
            Self {
                pet_id: path.pet_id,
            }
        }
    }

    /// Query parameters for show_pet_by_id operation
    #[derive(Deserialize)]
    pub struct Query {}

    /// Path parameters for show_pet_by_id operation
    #[derive(Deserialize)]
    pub struct Path {
        /// The id of the pet to retrieve
        pub pet_id: String,
    }

    #[derive(Serialize)]
    pub enum Response {
        Response200(Response200),
        Unspecified(HttpResponse),
    }

    /// Expected response to a valid request
    pub type Response200 = crate::components::schemas::Pet;
}

pub trait SwaggerPetstore: Clone {
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

#[derive(Clone)]
struct Server;
impl SwaggerPetstore for Server {
    type Error = std::io::Error;
}

fn main() -> std::io::Result<()> {
    let server = Server {};
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(resource("/pets").route(get().to(list_pets::<Server>)))
            .service(resource("/pets/{petId}").route(get().to(show_pet_by_id::<Server>)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
