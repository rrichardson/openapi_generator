use actix_web::http::Method;
use maplit::hashmap;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static SECURITY_MATRIX: Lazy<HashMap<(&str, Method), Vec<&str>>> = Lazy::new(|| {
    hashmap! {
    {{~#each paths as | _ path |}}
        {{~#with get}}
            ("{{regexify_path path}}", Method::GET) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with head}}
            ("{{regexify_path path}}", Method::HEAD) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with post}}
            ("{{regexify_path path}}", Method::POST) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with put}}
            ("{{regexify_path path}}", Method::PUT) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with delete}}
            ("{{regexify_path path}}", Method::DELETE) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with options}}
            ("{{regexify_path path}}", Method::OPTIONS) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with trace}}
            ("{{regexify_path path}}", Method::TRACE) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with patch}}
            ("{{regexify_path path}}", Method::PATCH) => vec![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
    {{~/each}}
        }
});
