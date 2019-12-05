#![allow(clippy::useless_format)]

use super::blocking::{{camelcase info.title "Client"}} as Client;
use super::*;
use mockito::{mock, Matcher};
use serde_json::json;
use crate::openapi_serialization::OpenapiSerialization;
use crate::example;

{{#each paths}}
  {{#each this}}
    {{~#each responses}}
      {{~#if (not (eq @key "default"))}}
        {{~> test_operation_client uri=@../../key operation_verb=@../key status=@key ../this response=this}}
      {{~/if}}
    {{/each}}
  {{/each}}
{{~/each}}
