use actix_web::http::Method;
use maplit::hashmap;
use maplit::hashset;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

pub static SECURITY_MATRIX: Lazy<HashMap<(&str, Method), HashSet<&str>>> = Lazy::new(|| {
    hashmap! {
    {{~#each paths as | _ path |}}
        {{~#with get}}
            (r"{{regexify_path path}}", Method::GET) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with head}}
            (r"{{regexify_path path}}", Method::HEAD) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with post}}
            (r"{{regexify_path path}}", Method::POST) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with put}}
            (r"{{regexify_path path}}", Method::PUT) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with delete}}
            (r"{{regexify_path path}}", Method::DELETE) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with options}}
            (r"{{regexify_path path}}", Method::OPTIONS) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with trace}}
            (r"{{regexify_path path}}", Method::TRACE) => hashset![
                {{~#each security}}
                    {{~#each auth as | scope |}}
                        "{{scope}}",
                    {{~/each}}
                {{~/each}}
            ],
        {{~/with}}
        {{~#with patch}}
            (r"{{regexify_path path}}", Method::PATCH) => hashset![
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
