use crate::models::*;

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
        parameters: &list_pets::Parameters,
    ) -> Result<list_pets::Response, surf::Exception> {
        let uri = format!("{uri}/pets", uri = self.uri);
        let mut response = surf::get(uri).set_query(&parameters.query())?.await?;
        use list_pets::Response::*;
        Ok(match response.status().as_str() {
            "200" => Response200(response.body_json().await?),
            _ => unimplemented!(),
        })
    }

    pub async fn show_pet_by_id(
        &self,
        parameters: &show_pet_by_id::Parameters,
    ) -> Result<show_pet_by_id::Response, surf::Exception> {
        let uri = format!(
            "{uri}/pets/{petId}",
            uri = self.uri,
            petId = parameters.pet_id
        );
        let mut response = surf::get(uri).set_query(&parameters.query())?.await?;
        use show_pet_by_id::Response::*;
        Ok(match response.status().as_str() {
            "200" => Response200(response.body_json().await?),
            _ => unimplemented!(),
        })
    }
}
