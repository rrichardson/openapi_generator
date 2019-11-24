mod helpers;
mod openapi_generator;

use crate::openapi_generator::OpenApiGenerator;
use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let template_dir = env::current_dir()?.join("template").join("template");
    let mut openapi_generator = OpenApiGenerator::new("openapi.yaml", &template_dir)
    .context(format!("cannot create OpenAPI generator with specifications from `openapi.yaml` and template from {}", template_dir.display()) )?;
    openapi_generator
        .render("output")
        .context("cannot render to `output`")?;
    Ok(())
}
