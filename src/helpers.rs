use anyhow::Result;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use json_pointer::JsonPointer;

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
handlebars_helper!(component_path: |ref_path: str| parse_component_path(ref_path));
handlebars_helper!(sanitize: |word: str| apply_sanitize(word));

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

pub(crate) fn apply_sanitize(word: &str) -> String {
    if KEYWORDS.iter().any(|&keyword| word == keyword) {
        format!("r#{}", word)
    } else {
        word.to_string()
    }
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
