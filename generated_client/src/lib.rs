#![allow(dead_code, unused_variables, clippy::clone_on_copy)]

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

    use serde::{Deserialize, Serialize};

    /// Parameters for list_pets operation
    #[derive(Deserialize)]
    pub struct Parameters {
        /// How many items to return at one time (max 100)
        pub limit: Option<i32>,
    }

    impl Parameters {
        pub fn new(query: Query, path: Path) -> Self {
            Self { limit: query.limit }
        }

        pub fn query(&self) -> Query {
            Query {
                limit: self.limit.clone(),
            }
        }

        pub fn path(&self) -> Path {
            Path {}
        }
    }

    /// Query parameters for list_pets operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {
        /// How many items to return at one time (max 100)
        pub limit: Option<i32>,
    }

    /// Path parameters for list_pets operation
    #[derive(Deserialize, Serialize)]
    pub struct Path {}

    #[derive(Deserialize, Serialize)]
    pub enum Response {
        Response200(Response200),
        // Unspecified(HttpResponse),
    }

    /// A paged array of pets
    pub type Response200 = crate::components::schemas::Pets;
}

pub mod create_pets {

    use serde::{Deserialize, Serialize};

    /// Parameters for create_pets operation
    #[derive(Deserialize)]
    pub struct Parameters {}

    impl Parameters {
        pub fn new(query: Query, path: Path) -> Self {
            Self {}
        }

        pub fn query(&self) -> Query {
            Query {}
        }

        pub fn path(&self) -> Path {
            Path {}
        }
    }

    /// Query parameters for create_pets operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {}

    /// Path parameters for create_pets operation
    #[derive(Deserialize, Serialize)]
    pub struct Path {}

    #[derive(Deserialize, Serialize)]
    pub enum Response {
        Response201(Response201),
        // Unspecified(HttpResponse),
    }

    /// Null response
    #[derive(Deserialize, Serialize)]
    pub struct Response201;
}

pub mod show_pet_by_id {

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

        pub fn query(&self) -> Query {
            Query {}
        }

        pub fn path(&self) -> Path {
            Path {
                pet_id: self.pet_id.clone(),
            }
        }
    }

    /// Query parameters for show_pet_by_id operation
    #[derive(Deserialize, Serialize)]
    pub struct Query {}

    /// Path parameters for show_pet_by_id operation
    #[derive(Deserialize, Serialize)]
    pub struct Path {
        /// The id of the pet to retrieve
        pub pet_id: String,
    }

    #[derive(Deserialize, Serialize)]
    pub enum Response {
        Response200(Response200),
        // Unspecified(HttpResponse),
    }

    /// Expected response to a valid request
    pub type Response200 = crate::components::schemas::Pet;
}

pub struct SwaggerPetstoreClient {
    uri: String,
}

impl SwaggerPetstoreClient {
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
        }
    }

    pub async fn list_pets(
        &self,
        parameters: list_pets::Parameters,
    ) -> Result<list_pets::Response, surf::Exception> {
        let uri = format!("{uri}/pets", uri = self.uri);
        surf::get(uri)
            .set_query(&parameters.query())?
            .recv_json()
            .await
    }

    pub async fn show_pet_by_id(
        &self,
        parameters: show_pet_by_id::Parameters,
    ) -> Result<show_pet_by_id::Response, surf::Exception> {
        let uri = format!(
            "{uri}/pets/{petId}",
            uri = self.uri,
            petId = parameters.pet_id
        );
        surf::get(uri)
            .set_query(&parameters.query())?
            .recv_json()
            .await
    }
}
