use super::*;
use mockito::{mock, Matcher};

{{~#each paths}}
{{#with get}}
#[test]
fn test_{{snakecase operationId}}() {
  let vizyr_api = VizyrApiClient::new(&mockito::server_url());

  let uri = format!("{{@../key}}"
    {{~#each parameters}}
      {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
    {{~/each~}}
  );

  let mock = mock("GET", uri)
    .match_body(Matcher::JsonString(
      r#"{"name":"name1","model":"model1"}"#.to_string(),
    ))
    .with_status(201)
    .with_body(r#""robot_token""#)
    .create();

  let parameters = {{snakecase operationId}}::Parameters {};

  let result = vizyr_api.{{snakecase operationId}}(&parameters);
  mock.assert();
}
{{~/with}}
{{~/each}}
