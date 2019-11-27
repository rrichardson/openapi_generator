use crate::models::*;
use async_std::task::block_on;

pub struct {{camelcase info.title "Client"}} {
    client: super::{{camelcase info.title "Client"}},
}

impl {{camelcase info.title "Client"}} {
    pub fn new(uri: &str) -> Self {
        Self { client: super::{{camelcase info.title "Client"}} { uri: uri.to_string() }}
    }
    {{~#each paths}}
    {{#with get}}
    pub fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        block_on(self.client.{{snakecase operationId}}(parameters))
    }
    {{~/with}}
    {{#with post}}
    pub fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        block_on(self.client.{{snakecase operationId}}(parameters))
    }
    {{~/with}}
    {{#with put}}
    pub fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        block_on(self.client.{{snakecase operationId}}(parameters))
    }
    {{~/with}}
    {{#with delete}}
    pub fn {{snakecase operationId}}(&self, parameters: &{{snakecase operationId}}::Parameters) -> Result<{{snakecase operationId}}::Response, surf::Exception> {
        block_on(self.client.{{snakecase operationId}}(parameters))
    }
    {{~/with}}
    {{~/each}}
}
