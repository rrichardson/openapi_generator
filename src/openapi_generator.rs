use crate::helpers::{camelcase, component_path, mixedcase, sanitize, snakecase};
use anyhow::{anyhow, Context, Result};
use handlebars::Handlebars;
use log;
use std::{
    env,
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
            template_path: template_path.as_ref().to_path_buf(),
        };
        let partials_path = env::current_dir()?.join("template").join("partials");
        openapi_generator
            .register_partials(&partials_path)
            .context(format!(
                "failed to register partials from `{}`",
                partials_path.display()
            ))?;
        openapi_generator.register_helpers();
        Ok(openapi_generator)
    }

    fn parse_specification(specs_path: &Path) -> Result<serde_yaml::Value> {
        let specs_string = std::fs::read_to_string(&specs_path).context(format!(
            "cannot read specification file `{}`",
            specs_path.display()
        ))?;
        serde_yaml::from_str(&specs_string).context(format!(
            "cannot parse specification file `{}`",
            specs_path.display()
        ))
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("camelcase", Box::new(camelcase));
        self.handlebars
            .register_helper("snakecase", Box::new(snakecase));
        self.handlebars
            .register_helper("mixedcase", Box::new(mixedcase));
        self.handlebars
            .register_helper("component_path", Box::new(component_path));
        self.handlebars
            .register_helper("sanitize", Box::new(sanitize));
    }

    fn register_partials<T: AsRef<Path>>(&mut self, partials_dir: T) -> Result<()> {
        for entry in walkdir::WalkDir::new(partials_dir) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    let template_name = path
                        .file_stem()
                        .ok_or_else(|| anyhow!("file name is empty"))?
                        .to_str()
                        .ok_or_else(|| anyhow!("file path is not unicode"))?;
                    self.handlebars
                        .register_template_file(template_name, path)
                        .context(format!("cannot register partial `{}`", path.display()))?;
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
            "cannot walk into template directory `{}`",
            template_path.display()
        ))? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_file() {
                    let template_key = &format!("{}", path.join(entry.file_name()).display());
                    self.handlebars
                        .register_template_file(template_key, entry.path())
                        .context(format!(
                            "cannot register template `{}` ",
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
                            "failed to render template `{}` at `{}`",
                            template_key,
                            output_file_path.display()
                        ))?;
                    log::info!("render {} to {}", template_key, output_file_path.display());
                } else if entry.file_type()?.is_dir() {
                    let mut path = path.to_path_buf();
                    path.push(entry.file_name());
                    let new_output_path = output_path.join(&path);
                    std::fs::create_dir_all(&new_output_path).context(format!(
                        "cannot create directory `{}`",
                        new_output_path.display()
                    ))?;
                    log::info!("create {}", new_output_path.display());
                    self.render_from_path(output_path, &path).context(format!(
                        "failed to render templates under `{}`",
                        new_output_path.display()
                    ))?;
                }
            }
        }
        Ok(())
    }
}
