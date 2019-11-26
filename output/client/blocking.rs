use crate::models::*;
use async_std::task::block_on;

pub struct SwaggerPetstoreClient {
    client: super::SwaggerPetstoreClient,
}

impl SwaggerPetstoreClient {
    pub fn new(uri: &str) -> Self {
        Self {
            client: super::SwaggerPetstoreClient {
                uri: uri.to_string(),
            },
        }
    }

    pub fn list_pets(
        &self,
        parameters: &list_pets::Parameters,
    ) -> Result<list_pets::Response, surf::Exception> {
        block_on(self.client.list_pets(parameters))
    }

    pub fn show_pet_by_id(
        &self,
        parameters: &show_pet_by_id::Parameters,
    ) -> Result<show_pet_by_id::Response, surf::Exception> {
        block_on(self.client.show_pet_by_id(parameters))
    }
}
