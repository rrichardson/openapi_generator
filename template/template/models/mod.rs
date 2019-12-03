#![allow(clippy::clone_on_copy)]

pub mod components {
{{~#with components}}
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};

        {{~#each schemas}}
            {{>schema name=@key this}}
        {{~/each}}
    }
{{~/with}}
}
{{#each paths}}
    {{~>operation_types get}}
    {{~>operation_types head}}
    {{~>operation_types post}}
    {{~>operation_types put}}
    {{~>operation_types delete}}
    {{~>operation_types options}}
    {{~>operation_types trace}}
    {{~>operation_types patch}}
{{~/each}}