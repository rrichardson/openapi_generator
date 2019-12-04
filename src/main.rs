mod helpers;
mod openapi_generator;

use crate::openapi_generator::OpenApiGenerator;
use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    openapi: String,
    #[structopt(short = "d", long = "dest", default_value = "output")]
    /// Destination directory of the generated code
    destination: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Cli::from_args();
    let template_dir = env::current_dir()?.join("template").join("template");
    let mut openapi_generator = OpenApiGenerator::new(args.openapi, &template_dir)
    .context(format!("cannot create OpenAPI generator with specifications from `openapi.yaml` and template from {}", template_dir.display()) )?;
    openapi_generator
        .render(args.destination.clone())
        .context(format!(
            "cannot render to `{}`",
            args.destination.to_string_lossy()
        ))?;
    Ok(())
}
