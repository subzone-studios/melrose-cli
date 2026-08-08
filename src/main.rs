use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result, bail};

use crate::{
    cli::{Cli, GenerateCommand, GenerateSubcommand},
    domain::GeneratorContext,
};

mod cli;
mod domain;

const BASE_PATH: &'static str = "src";

const SERVER_FEATURE_FOLDERS: &[&str] = &["services", "components", "lib"];
const CLIENT_FEATURE_FOLDERS: &[&str] = &["controllers", "components", "lib"];
const SHARED_FEATURE_FOLDERS: &[&str] = &["components", "lib"];

fn main() -> Result<()> {
    let Some(cwd) = env::current_dir().ok() else {
        bail!("No working directory found")
    };

    let base_path = PathBuf::from(cwd);
    let base_path = base_path.join(BASE_PATH);

    match cli::Cli::parse_args() {
        Cli::Generate(shim!(Generate -> Feature {name, context})) => {
            let feature_path = base_path.join(context.as_ref()).join("features").join(name);
            if feature_path.exists() && feature_path.is_dir() {
                bail!("Directory '{feature_path:?}' already exists")
            }

            fs::create_dir_all(&feature_path).context("Failed to create feature directory")?;

            match context {
                GeneratorContext::Client => CLIENT_FEATURE_FOLDERS
                    .iter()
                    .try_for_each(|path| fs::create_dir_all(&feature_path.join(path)))?,
                GeneratorContext::Server => SERVER_FEATURE_FOLDERS
                    .iter()
                    .try_for_each(|path| fs::create_dir_all(&feature_path.join(path)))?,
                GeneratorContext::Shared => SHARED_FEATURE_FOLDERS
                    .iter()
                    .try_for_each(|path| fs::create_dir_all(&feature_path.join(path)))?,
            }
        }

        Cli::Generate(shim!(Generate -> Component {component_name, feature_name, context})) => {
            let feature_path = base_path
                .join(context.as_ref())
                .join("features")
                .join(feature_name);
            if !feature_path.exists() || !feature_path.is_dir() {
                bail!("Directory '{feature_path:?}' does not exist")
            }

            fs::create_dir_all(&feature_path.join("components"))?;

            fs::write(
                &feature_path
                    .join("components")
                    .join(format!("{component_name}.ts")),
                "",
            )?;
        }
        Cli::Generate(shim!(Generate -> Service {feature_name, service_name})) => {
            let feature_path = base_path.join("server").join("features").join(feature_name);
            if !feature_path.exists() || !feature_path.is_dir() {
                bail!("Directory '{feature_path:?}' does not exist")
            }

            fs::create_dir_all(&feature_path.join("services"))?;

            fs::write(
                &feature_path
                    .join("services")
                    .join(format!("{service_name}.ts")),
                "",
            )?;
        }
        Cli::Generate(shim!(Generate -> Controller {feature_name, controller_name})) => {
            let feature_path = base_path.join("client").join("features").join(feature_name);
            if !feature_path.exists() || !feature_path.is_dir() {
                bail!("Directory '{feature_path:?}' does not exist")
            }

            fs::create_dir_all(&feature_path.join("controllers"))?;

            fs::write(
                &feature_path
                    .join("controllers")
                    .join(format!("{controller_name}.ts")),
                "",
            )?;
        }
    }

    Ok(())
}
