use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde_yaml::{Mapping, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

const HEADER: &str = "# Managed by tuggo CLI - DO NOT EDIT MANUALLY";
const GENERATED: &str = ".tuggo/compose.generated.yml";

#[derive(Parser)]
#[command(
    name = "tuggo",
    version,
    about = "Reproduce Docker Compose environments locally"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compose {
        env: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        args: Vec<String>,
    },
}

fn compose_yaml(source: &str, env: &str) -> Result<String> {
    let root: Value = serde_yaml::from_str(source).context("Invalid tuggo.yml")?;
    let environments = root
        .as_mapping()
        .context("tuggo.yml must be a YAML mapping")?;
    let selected = environments
        .get(Value::String(env.to_owned()))
        .with_context(|| format!("Environment '{env}' not found in tuggo.yml"))?;
    let mapping = selected
        .as_mapping()
        .context("Environment must contain a Compose mapping")?;
    if !mapping.contains_key(Value::String("services".into())) {
        bail!("Environment '{env}' must define services");
    }
    let mut output = Mapping::new();
    for (key, value) in mapping {
        output.insert(key.clone(), value.clone());
    }
    Ok(format!("{HEADER}\n{}", serde_yaml::to_string(&output)?))
}

fn write_managed(path: &Path, content: &str) -> Result<()> {
    if path.exists() {
        let existing =
            fs::read_to_string(path).with_context(|| format!("Cannot read {}", path.display()))?;
        if existing.lines().next() != Some(HEADER) {
            bail!("Refusing to overwrite unmanaged file: {}", path.display());
        }
    }
    let parent = path.parent().context("Generated file has no parent")?;
    fs::create_dir_all(parent)?;
    let temp = parent.join(format!(".compose-{}.tmp", std::process::id()));
    fs::write(&temp, content)?;
    if let Err(err) = fs::rename(&temp, path) {
        let _ = fs::remove_file(&temp);
        return Err(err.into());
    }
    Ok(())
}

fn run(env: &str, args: &[String]) -> Result<i32> {
    if !env
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        || env.is_empty()
    {
        bail!("Environment name must contain only letters, digits, '-' or '_'");
    }
    let root = std::env::current_dir()?;
    let source = fs::read_to_string(root.join("tuggo.yml")).context("Cannot read tuggo.yml")?;
    let yaml = compose_yaml(&source, env)?;
    let generated = root.join(GENERATED);
    write_managed(&generated, &yaml)?;
    let project = format!(
        "{}-{}",
        root.file_name().unwrap_or_default().to_string_lossy(),
        env
    )
    .to_ascii_lowercase()
    .replace('_', "-");
    let env_file: PathBuf = root.join(format!(".env.{env}"));
    let mut command = Command::new("docker");
    command
        .arg("compose")
        .arg("--project-directory")
        .arg(&root)
        .arg("-p")
        .arg(project)
        .arg("-f")
        .arg(&generated);
    if env_file.is_file() {
        command.arg("--env-file").arg(env_file);
    }
    let status = command
        .args(args)
        .status()
        .context("Failed to run docker compose")?;
    Ok(status.code().unwrap_or(1))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Compose { env, args } => run(&env, &args),
    };
    match result {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        Err(error) => {
            eprintln!("tuggo: {error:#}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_environment() {
        let yaml = "dev:\n  services:\n    web:\n      image: nginx:alpine\nprod:\n  services:\n    web:\n      image: nginx:stable\n";
        let result = compose_yaml(yaml, "dev").unwrap();
        assert!(result.starts_with(HEADER));
        assert!(result.contains("nginx:alpine"));
        assert!(!result.contains("nginx:stable"));
    }
    #[test]
    fn rejects_missing_environment() {
        assert!(compose_yaml("dev:\n  services: {}\n", "prod").is_err());
    }
    #[test]
    fn refuses_unmanaged_file() {
        let path =
            std::env::temp_dir().join(format!("tuggo-guard-{}-{}", std::process::id(), "test.yml"));
        fs::write(&path, "services: {}\n").unwrap();
        assert!(write_managed(&path, &format!("{HEADER}\nservices: {{}}\n")).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "services: {}\n");
        fs::remove_file(path).unwrap();
    }
}
