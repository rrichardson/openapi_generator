use crate::helpers::{
    camelcase, component_path, has, is_http_code_success, json, mixedcase, prefix_lines,
    sane_camelcase, sane_snakecase, sanitize, shoutysnakecase, snakecase, str_eq,
};
use anyhow::{anyhow, Context, Result};
use handlebars::Handlebars;
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
            template_path: template_path.as_ref().join("template"),
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
        Self::fixup_specs(specs)?;
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

    // Inspect each path item. If there is no `operationId`, synthesize one
    fn fixup_specs(specs: &mut serde_yaml::Mapping) -> Result<()> {
        use serde_yaml::Value as V;
        let components = specs
            .get_mut(&V::String("components".into()))
            .ok_or(anyhow!("Spec is missing the 'components' property"))?
            .as_mapping_mut()
            .ok_or(anyhow!("'path' property is not a Mapping"))?
            .to_owned();
        let paths = specs
            .get_mut(&V::String("paths".into()))
            .ok_or(anyhow!("Spec is missing the 'paths' property"))?
            .as_mapping_mut()
            .ok_or(anyhow!("'paths' property is not a Mapping"))?;
        for (path, methods) in paths.iter_mut() {
            let methods = methods
                .as_mapping_mut()
                .ok_or(anyhow!("path body is not a Mapping"))?;
            for (method, ref mut body) in methods {
                let body = body
                    .as_mapping_mut()
                    .ok_or(anyhow!("method body is not a Mapping"))?;
                let pathname = path.as_str().ok_or(anyhow!("path key is not a string"))?;
                let method = method
                    .as_str()
                    .ok_or(anyhow!("method key is not a string"))?;
                Self::inject_operation_id(body, method, pathname)?;
                Self::resolve_param_refs(body, &components)?;
            }
        }
        Ok(())
    }

    fn inject_operation_id(
        path_body: &mut serde_yaml::Mapping,
        method: &str,
        pathname: &str,
    ) -> Result<()> {
        use serde_yaml::Value as V;
        if path_body.get(&V::String("operationId".into())).is_none() {
            path_body.insert(
                V::String("operationId".into()),
                V::String(Self::path_to_typename(method, pathname)),
            );
        }
        Ok(())
    }

    fn path_to_typename(method: &str, path: &str) -> String {
        use heck::SnakeCase;
        let p = path
            .replacen('/', "", 1)
            .replace('/', "_")
            .replace('.', "_");
        format!("{}_{}", method, p).to_snake_case()
    }

    // searches through the parameters sequence for references to components/parameters
    // and injects the parameter contents into the path body
    fn resolve_param_refs(
        path_body: &mut serde_yaml::Mapping,
        components: &serde_yaml::Mapping,
    ) -> Result<()> {
        use serde_yaml::Value as V;
        let parameters_key = V::String("parameters".into());
        if path_body.get(&parameters_key).is_none() {
            return Ok(());
        }
        let parameters = path_body
            .get_mut(&parameters_key)
            .ok_or(anyhow!("Path body is missing the 'parameters' property"))?
            .as_sequence_mut()
            .ok_or(anyhow!("'parameters' property is not a Sequence"))?;
        let mut changed = false;
        let changed_ref = &mut changed;
        let new_params: Vec<serde_yaml::Value> = parameters
            .iter()
            .map(|p| {
                let param = p
                    .as_mapping()
                    .ok_or(anyhow!("parameter is not a Mapping"))?;
                if let Some(name) = param.get(&V::String("$ref".into())) {
                    let remote = Self::find_parameter(
                        components,
                        name.as_str().expect("$ref value should be a string"),
                    )?
                    .to_owned();
                    *changed_ref = true;
                    Ok(V::Mapping(remote))
                } else {
                    Ok(V::Mapping(param.to_owned()))
                }
            })
            .collect::<Result<_>>()?;
        if changed {
            path_body.remove(&parameters_key);
            path_body.insert(parameters_key, V::Sequence(new_params));
        }
        Ok(())
    }

    fn find_parameter<'a>(
        components: &'a serde_yaml::Mapping,
        reference: &str,
    ) -> Result<&'a serde_yaml::Mapping> {
        use serde_yaml::Value as V;
        let type_name = reference
            .rsplit('/')
            .next()
            .ok_or(anyhow!("failed to get parameter name from $ref"))?;
        let parameters = components
            .get(&V::String("parameters".into()))
            .ok_or(anyhow!(
                "spec.components is missing the 'parameters' property"
            ))?
            .as_mapping()
            .ok_or(anyhow!("'parameters' property is not a Mapping"))?;
        parameters
            .get(&V::String(type_name.into()))
            .ok_or(anyhow!(
                "spec.components.parameters does not have key: '{}'",
                type_name
            ))?
            .as_mapping()
            .ok_or(anyhow!("parameter '{}' is not a Mapping", type_name))
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("camelcase", Box::new(camelcase));
        self.handlebars
            .register_helper("snakecase", Box::new(snakecase));
        self.handlebars
            .register_helper("shoutysnakecase", Box::new(shoutysnakecase));
        self.handlebars
            .register_helper("sane_snakecase", Box::new(sane_snakecase));
        self.handlebars
            .register_helper("sane_camelcase", Box::new(sane_camelcase));
        self.handlebars
            .register_helper("mixedcase", Box::new(mixedcase));
        self.handlebars
            .register_helper("component_path", Box::new(component_path));
        self.handlebars
            .register_helper("sanitize", Box::new(sanitize));
        self.handlebars
            .register_helper("prefix_lines", Box::new(prefix_lines));
        self.handlebars.register_helper("str_eq", Box::new(str_eq));
        self.handlebars.register_helper("has", Box::new(has));
        self.handlebars.register_helper("json", Box::new(json));
        self.handlebars
            .register_helper("is_http_code_success", Box::new(is_http_code_success));
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
