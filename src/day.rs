use std::{io::Write, path::PathBuf, str::FromStr};

use serde::Serialize;
use thiserror::Error;
use tinytemplate::TinyTemplate;
use toml_edit::Document;

use crate::config::Config;

const EXPECT_PACKAGE: &str = env!("CARGO_PKG_NAME");

fn render_templates_into(
    current_dir: &PathBuf,
    day_dir: &PathBuf,
    day: u8,
    day_name: &str,
) -> Result<(), Error> {
    #[derive(Serialize)]
    struct Context {
        day: u8,
        package_name: String,
    }

    let context = Context {
        day,
        package_name: day_name.to_string(),
    };

    let template_dir = current_dir.join("day-template");

    for template in &["Cargo.toml", "src/lib.rs", "src/main.rs"] {
        let mut tt = TinyTemplate::new();
        let template_text =
            std::fs::read_to_string(template_dir.join(format!("{}.tmpl", template)))?;
        tt.add_template(&template, &template_text)
            .map_err(|err| Error::Template(err, template.to_string()))?;

        let rendered_text = tt
            .render(template, &context)
            .map_err(|err| Error::Template(err, template.to_string()))?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(day_dir.join(template))?;
        file.write_all(rendered_text.as_bytes())?;
    }
    Ok(())
}

fn add_create_to_workspace(current_dir: &PathBuf, crate_name: &str) -> Result<(), Error> {
    let cargo_toml_path = current_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        Err(Error::NoCargoToml)?;
    }

    let mut manifest = Document::from_str(&std::fs::read_to_string(&cargo_toml_path)?)?;

    fn get_package_name(manifest: &Document) -> Option<&str> {
        manifest
            .as_table()
            .get("package")?
            .as_table_like()?
            .get("name")?
            .as_value()?
            .as_str()
    }

    let package_name = get_package_name(&manifest).ok_or(Error::MalformedToml)?;

    if package_name != EXPECT_PACKAGE {
        Err(Error::WrongPackage(package_name.to_string()))?;
    }

    let root_table = manifest.as_table_mut();
    let workspace = root_table
        .entry("workspace")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()));
    let workspace = workspace.as_table_mut().ok_or(Error::MalformedToml)?;
    let members =
        workspace
            .entry("members")
            .or_insert(toml_edit::Item::Value(toml_edit::Value::Array(
                Default::default(),
            )));
    let members = members
        .as_value_mut()
        .ok_or(Error::MalformedToml)?
        .as_array_mut()
        .ok_or(Error::MalformedToml)?;

    if members.iter().any(|item| {
        item.as_str()
            .map(|item_str| item_str == crate_name)
            .unwrap_or_default()
    }) {
        Err(Error::CrateAlreadyExists(crate_name.to_string()))?;
    }

    members.push(crate_name);

    std::fs::write(cargo_toml_path, manifest.to_string())?;
    Ok(())
}

pub fn initialize(_config: &Config, day: u8, force: bool) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let day_name = format!("day{:02}", day);
    let day_dir = current_dir.join(&day_name);

    if day_dir.exists() && !force {
        Err(Error::DayExist(day))?;
    }
    std::fs::create_dir_all(day_dir.join("src"))?;

    add_create_to_workspace(&current_dir, &day_name)?;

    // render templates, creating new sub-crate
    render_templates_into(&current_dir, &day_dir, day, &day_name)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("directory for day {0} already exists")]
    DayExist(u8),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("template error for {1}")]
    Template(#[source] tinytemplate::error::Error, String),
    #[error("Cargo.toml not found")]
    NoCargoToml,
    #[error("could not parse Cargo.toml")]
    ParseToml(#[from] toml_edit::TomlError),
    #[error("Cargo.toml is malformed")]
    MalformedToml,
    #[error(
        "working dir must be root of package {} but is actually {0}",
        EXPECT_PACKAGE
    )]
    WrongPackage(String),
    #[error("crate already exists in workspace: {0}")]
    CrateAlreadyExists(String),
}
