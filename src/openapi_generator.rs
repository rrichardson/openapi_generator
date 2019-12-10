use crate::helpers::{
    camelcase, component_path, has, json, mixedcase, sanitize, shoutysnakecase, snakecase, has_field
};
use anyhow::{anyhow, Context, Result};
use handlebars::Handlebars;
use log;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub struct OpenApiGenerator {
    handlebars: Handlebars,
    specs: serde_yaml::Value,
    template_path: PathBuf,
}

impl OpenApiGenerator {
    pub fn new<T: AsRef<Path>, U: AsRef<Path>>(specs_path: T, template_path: U) -> Result<Self> {
        let mut openapi_generator = Self {
            handlebars: Handlebars::new(),
            specs: Self::parse_specification(&specs_path.as_ref())?,
            template_path: template_path.as_ref().join("template").to_path_buf(),
        };
        let partials_path = template_path.as_ref().join("partials");
        openapi_generator
            .register_partials(&partials_path)
            .context(format!(
                "Failed to register partials from `{}`",
                partials_path.display()
            ))?;
        openapi_generator.register_helpers();
        let specs = openapi_generator
            .specs
            .as_mapping_mut()
            .context("specification is not a mapping")?;
        specs.insert(
            serde_yaml::Value::String("openapi_generator_version".to_string()),
            serde_yaml::Value::String(env!("CARGO_PKG_VERSION").to_string()),
        );
        Ok(openapi_generator)
    }

    fn parse_specification(specs_path: &Path) -> Result<serde_yaml::Value> {
        let specs_string = std::fs::read_to_string(&specs_path).context(format!(
            "Cannot read specification file `{}`",
            specs_path.display()
        ))?;
        serde_yaml::from_str(&specs_string).context(format!(
            "Cannot parse specification file `{}`",
            specs_path.display()
        ))
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("camelcase", Box::new(camelcase));
        self.handlebars
            .register_helper("snakecase", Box::new(snakecase));
        self.handlebars
            .register_helper("shoutysnakecase", Box::new(shoutysnakecase));
        self.handlebars
            .register_helper("mixedcase", Box::new(mixedcase));
        self.handlebars
            .register_helper("component_path", Box::new(component_path));
        self.handlebars
            .register_helper("sanitize", Box::new(sanitize));
        self.handlebars.register_helper("has", Box::new(has));
        self.handlebars.register_helper("has_field", Box::new(has_field));
        self.handlebars.register_helper("json", Box::new(json));
    }

    fn register_partials<T: AsRef<Path>>(&mut self, partials_dir: T) -> Result<()> {
        for entry in walkdir::WalkDir::new(partials_dir) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    let template_name = path
                        .file_stem()
                        .ok_or_else(|| anyhow!("File name is empty"))?
                        .to_str()
                        .ok_or_else(|| anyhow!("File path is not unicode"))?;
                    self.handlebars
                        .register_template_file(template_name, path)
                        .context(format!("Cannot register partial `{}`", path.display()))?;
                    log::info!(
                        "new partial registered: {} ({})",
                        template_name,
                        path.display()
                    );
                }
            }
        }
        Ok(())
    }

    pub fn render<T: AsRef<Path>>(&mut self, output_path: T) -> Result<()> {
        self.render_from_path(output_path.as_ref(), &PathBuf::new())
    }

    fn render_from_path(&mut self, output_path: &Path, path: &Path) -> Result<()> {
        let template_path = self.template_path.join(path);
        for entry in std::fs::read_dir(&template_path).context(format!(
            "Cannot walk into template directory `{}`",
            template_path.display()
        ))? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_file() {
                    let template_key = &format!("{}", path.join(entry.file_name()).display());
                    self.handlebars
                        .register_template_file(template_key, entry.path())
                        .context(format!(
                            "Cannot register template `{}` ",
                            entry.path().display()
                        ))?;
                    log::info!(
                        "new template registered: {} ({})",
                        template_key,
                        entry.path().display()
                    );
                    let output_file_path = output_path.join(path).join(entry.file_name());
                    let mut output_file = File::create(&output_file_path)?;
                    self.handlebars
                        .render_to_write(template_key, &self.specs, &mut output_file)
                        .context(format!(
                            "Failed to render template `{}` at `{}`",
                            template_key,
                            output_file_path.display()
                        ))?;
                    log::info!("render {} to {}", template_key, output_file_path.display());
                } else if entry.file_type()?.is_dir() {
                    let mut path = path.to_path_buf();
                    path.push(entry.file_name());
                    let new_output_path = output_path.join(&path);
                    std::fs::create_dir_all(&new_output_path).context(format!(
                        "Cannot create directory `{}`",
                        new_output_path.display()
                    ))?;
                    log::info!("create {}", new_output_path.display());
                    self.render_from_path(output_path, &path).context(format!(
                        "Failed to render templates under `{}`",
                        new_output_path.display()
                    ))?;
                }
            }
        }
        Ok(())
    }
}
