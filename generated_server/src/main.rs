#![allow(dead_code, unused_variables)]

use actix_web::{web::*, App, HttpResponse, HttpServer, Responder};

pub mod components {
    pub mod schemas {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct Error {
            pub code: Option<i32>,
            pub message: Option<String>,
        }

        #[derive(Deserialize)]
        pub struct Pet {
            pub id: Option<i64>,
            pub name: Option<String>,
            pub tag: Option<String>,
        }

        #[derive(Deserialize)]
        pub struct Pets {
            pub data: Option,
        }

        #[derive(Deserialize)]
        pub struct Pets {
            pub data: Option<Vec<PetsItem>>,
        }
    }
}

pub mod list_pets {
    use super::components;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Limit {
        pub data: Option,
    }

    #[derive(Deserialize)]
    pub struct Limit {
        pub data: Option<Vec<LimitItem>>,
    }

    #[derive(Deserialize)]
    pub struct Query {
        /// How many items to return at one time (max 100)
        pub limit: Option<Limit>,
    }

    #[derive(Deserialize)]
    pub struct Response200 {
        pub data: Option<components::schemas::Pets>,
    }

    #[derive(Deserialize)]
    pub struct Response404 {
        pub data: Option,
    }

    #[derive(Deserialize)]
    pub struct ResponseDefault {
        pub data: Option<components::schemas::Error>,
    }
}

pub mod create_pets {
    use super::components;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Query {}

    #[derive(Deserialize)]
    pub struct Response201 {
        pub data: Option,
    }

    #[derive(Deserialize)]
    pub struct ResponseDefault {
        pub data: Option<components::schemas::Error>,
    }
}

pub mod show_pet_by_id {
    use super::components;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct PetId {
        pub data: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Query {}

    #[derive(Deserialize)]
    pub struct Response200 {
        pub data: Option<components::schemas::Pet>,
    }

    #[derive(Deserialize)]
    pub struct ResponseDefault {
        pub data: Option<components::schemas::Error>,
    }
}

/// List all pets
fn list_pets(query: Query<list_pets::Query>) -> impl Responder {
    HttpResponse::Ok().body("List all pets")
}

/// Create a pet
fn create_pets() -> impl Responder {
    HttpResponse::Ok().body("Create a pet")
}

/// Info for a specific pet
fn show_pet_by_id(query: Query<show_pet_by_id::Query>) -> impl Responder {
    HttpResponse::Ok().body("Info for a specific pet")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                resource("/pets")
                    .route(get().to(list_pets))
                    .route(post().to(create_pets)),
            )
            .service(resource("/pets/{petId}").route(get().to(show_pet_by_id)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
