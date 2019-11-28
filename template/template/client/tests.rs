use super::blocking::{{camelcase info.title "Client"}} as Client;
use super::*;
use mockito::{mock, Matcher};
use serde_json::json;
use crate::openapi_serialization::OpenapiSerialization;

{{~#each paths}}
{{#with get}}
#[test]
fn test_{{snakecase operationId}}() {
  let api_client = Client::new(&mockito::server_url());

  let uri = format!("{{@../key}}"
    {{~#each parameters}}
      {{~#if (eq in "path")}}, {{name}} = {{>example}}{{/if}}
    {{~/each~}}
  );

  let mock = mock("GET", Matcher::Exact(uri))
    .match_query(Matcher::AllOf(vec![
      {{~#each parameters}}
        {{~#if (eq in "query")}}
        Matcher::UrlEncoded("{{name}}".into(), ({{>example}}).serialize().unwrap()),
        {{/if}}
      {{~/each~}}
    ]))
    .with_status(200)
    .create();

  let parameters = {{snakecase operationId}}::Parameters {
    {{~#each parameters}}
      {{~#if (eq in "path")}}
      {{name}}: {{>example}},
      {{/if}}
      {{~#if (eq in "query")}}
      {{name}}: {{>example}},
      {{/if}}
    {{~/each~}}
  };

  let result = api_client.{{snakecase operationId}}(&parameters);
  mock.assert();
}
{{~/with}}
{{~/each}}
