use anyhow::{Context, Result};
use handlebars::handlebars_helper;
use handlebars::{
    Context as HandlebarsContext, Handlebars, Helper, JsonRender, Output, RenderContext,
    RenderError,
};
use json_pointer::JsonPointer;
use std::fs::File;

fn parse_component_path(ref_path: &str) -> String {
    let mut path = Vec::new();
    let mut pointer = ref_path.parse::<JsonPointer<_, _>>().unwrap();
    while let Some(segment) = pointer.pop() {
        path.push(segment);
    }
    path.reverse();
    path.join("::")
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

macro_rules! case_helper {
    ($name:ident, $function:ident) => {
        fn $name(
            helper: &Helper,
            _: &Handlebars,
            _: &HandlebarsContext,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            use heck::*;
            let values = helper
                .params()
                .iter()
                .map(|v| v.value().render())
                .map(|v| v.to_snake_case())
                .collect::<Vec<_>>();
            let rendered = values.as_slice().join("_").$function();
            out.write(rendered.as_ref())?;
            Ok(())
        }
    };
}

case_helper!(mixedcase, to_mixed_case);
case_helper!(camelcase, to_camel_case);
case_helper!(snakecase, to_snake_case);
handlebars_helper!(component_path: |ref_path: str| parse_component_path(ref_path));

fn main() -> Result<()> {
    pretty_env_logger::init();
    let specs = openapi::from_path("openapi.yaml")
        .map_err(failure::Fail::compat)
        .context("failed to parse openapi.yaml")?;
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("camelcase", Box::new(camelcase));
    handlebars.register_helper("snakecase", Box::new(snakecase));
    handlebars.register_helper("mixedcase", Box::new(mixedcase));
    handlebars.register_helper("component_path", Box::new(component_path));
    handlebars.register_template_file("main.rs", "template/main.rs.hbs")?;
    handlebars.register_template_file("schema", "template/schema.rs.hbs")?;
    handlebars.register_template_file("operation_types", "template/operation_types.rs.hbs")?;
    handlebars.register_template_file("data_type", "template/data_type.rs.hbs")?;
    let mut output_file = File::create("generated_server/src/main.rs")?;
    println!("{}", serde_json::to_string(&specs).unwrap());
    handlebars.render_to_write("main.rs", &specs, &mut output_file)?;
    Ok(())
}
