pub mod components {
{{~#with components}}
    pub mod schemas {
      use crate::components;
      use serde_json::json;
      use crate::components::schemas::{
        {{~#each schemas}}
        {{camelcase @key suffix}},
        {{~#if (eq type "object")~}}
            {{~#each properties}}
                {{~#if (eq type "object")}}
                    {{camelcase @../key @key}},

                {{/if}}
                {{~#if (eq type "array")}}
                    {{~#if (eq items.type "object")}}
                        {{camelcase @../key @key "item"}},

                    {{/if}}
                {{~/if}}
            {{~/each}}
        {{~/if}}
        {{~/each}}
      };

      {{~#each schemas}}
        {{>schema_example name=@key this}}
      {{~/each}}
    }
{{~/with}}
}