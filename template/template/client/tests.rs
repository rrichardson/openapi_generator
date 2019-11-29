use super::blocking::{{camelcase info.title "Client"}} as Client;
use super::*;
use mockito::{mock, Matcher};
use serde_json::json;
use crate::openapi_serialization::OpenapiSerialization;

{{~#each paths}}
  {{#with get}}{{> test_operation_client operation_verb="get"}}{{~/with}}
{{~/each}}
