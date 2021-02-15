use anyhow::Result;
use english_numbers;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use json_pointer::JsonPointer;
use serde_json::value::Value as Json;

macro_rules! case_helper {
    ($name:ident, $function:ident) => {
        pub(crate) fn $name(
            helper: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            use heck::*;
            let values = helper
                .params()
                .iter()
                .map(|v| v.value().render())
                .collect::<Vec<_>>();
            let rendered = values.as_slice().join(" ").$function();
            out.write(rendered.as_ref())?;
            Ok(())
        }
    };
}

case_helper!(mixedcase, to_mixed_case);
case_helper!(camelcase, to_camel_case);
case_helper!(snakecase, to_snake_case);
case_helper!(shoutysnakecase, to_shouty_snake_case);
handlebars_helper!(sane_snakecase: |word: str| to_sane_snakecase(word));
handlebars_helper!(sane_camelcase: |word: str| to_sane_camelcase(word));
handlebars_helper!(str_eq: |a: str, b: str| a == b);
handlebars_helper!(is_reserved: |word: str| is_reserved_word(word));
handlebars_helper!(component_path: |ref_path: str| parse_component_path(ref_path));
handlebars_helper!(sanitize: |word: str| apply_sanitize(word));
handlebars_helper!(json: |data: Json| apply_json(data));
handlebars_helper!(prefix_lines: |prefix: str, data: str| insert_prefix_lines(
    prefix, data
));
handlebars_helper!(is_http_code_success: |http_status: str| http_status.starts_with('1') || http_status.starts_with('2') || http_status.starts_with('3'));

pub(crate) fn insert_prefix_lines(prefix: &str, data: &str) -> String {
    let v = data
        .lines()
        .map(|line| format!("{} {}", prefix, line))
        .collect::<Vec<String>>();
    v.join("\n")
}

pub(crate) fn parse_component_path(ref_path: &str) -> String {
    use heck::CamelCase;
    let mut path = Vec::new();
    let mut pointer = ref_path.parse::<JsonPointer<_, _>>().unwrap();
    while let Some(segment) = pointer.pop() {
        path.push(segment);
    }
    if let Some(name) = path.first_mut() {
        *name = name.to_camel_case()
    }
    path.reverse();
    path.join("::")
}

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "yield", "async", "await", "try",
];

pub(crate) fn is_reserved_word(word: &str) -> bool {
    KEYWORDS.iter().any(|&keyword| word == keyword)
}

pub(crate) fn apply_sanitize(word: &str) -> String {
    if is_reserved_word(word) {
        format!("{}_", word)
    } else {
        word.to_string()
    }
}

pub(crate) fn to_sane_snakecase(word: &str) -> String {
    use heck::SnakeCase;
    // This assumes URI encoding, which is basically ascii
    if word.len() == 1 {
        let ch = word.chars().next().unwrap();
        if ch == ch.to_ascii_uppercase() {
            format!("{}_", ch.to_ascii_lowercase())
        } else {
            word.to_string()
        }
    } else {
        apply_sanitize(word.to_snake_case().as_str())
    }
}

pub(crate) fn to_sane_camelcase(word: &str) -> String {
    use heck::CamelCase;
    use regex::Regex;
    // This assumes URI encoding, which is basically ascii
    if word.chars().next().unwrap().is_digit(10) {
        let re = Regex::new(r"(\d+)(\w+)").unwrap();
        let caps = re.captures(word).unwrap();
        let pre = caps.get(1).map_or("", |m| m.as_str());
        let post = caps.get(2).map_or("", |m| m.as_str());
        let num = pre.parse::<i64>().unwrap();
        let res = english_numbers::convert(
            num,
            english_numbers::Formatting {
                title_case: true,
                ..Default::default()
            },
        );
        format!("{}{}", res, post)
    } else {
        word.to_camel_case()
    }
}

pub(crate) fn has(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let data = helper
        .param(0)
        .ok_or_else(|| RenderError::new("data not found"))?
        .value();
    let field = helper
        .param(1)
        .ok_or_else(|| RenderError::new("field not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("field is not a string"))?;
    let value = helper.param(2);
    let result = match data {
        serde_json::Value::Array(data) => {
            if let Some(value) = value {
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                data.iter()
                    .any(|list_elem| list_elem[field] == value_converted)
            } else {
                data.iter().any(|list_elem| list_elem == field)
            }
        }
        serde_json::Value::Object(data) => {
            if let Some(value) = value {
                let field_value = data
                    .get(field)
                    .ok_or_else(|| RenderError::new("field does not exist"))?;
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                field_value == value_converted
            } else {
                data.get(field).is_some()
            }
        }
        _ => false,
    };
    out.write(if result { "true" } else { "" })?;
    Ok(())
}

pub(crate) fn apply_json(data: &Json) -> String {
    data.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_component_path() {
        assert_eq!(
            parse_component_path("#/components/schemas/Pet"),
            "components::schemas::Pet".to_string()
        )
    }
}
