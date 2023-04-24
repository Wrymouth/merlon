use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, path::Path, io::{BufReader, BufWriter}};
use anyhow::Result;
use heck::AsKebabCase;
use serde::{Deserialize, Serialize};

// TODO: use taplo instead of toml to preserve comments etc

/// `merlon.toml` file. This file is used to store metadata about a mod.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The rev of the papermario submodule that this mod is based on
    pub base_commit: String,

    /// Package metadata
    pub package: Package,

    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    license: String,
    keywords: Vec<String>,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validate package metadata, returning a list of errors
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.name.is_empty() {
            errors.push("name cannot be empty".to_owned());
        }
        if format!("{}", AsKebabCase(&self.name)) != self.name {
            errors.push("name must be kebab-case".to_owned());
        }
        if self.name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '-') {
            errors.push("name must be alphanumeric".to_owned());
        }
        if self.version.is_empty() {
            errors.push("version cannot be empty".to_owned());
        }
        // TODO: validate version
        if self.authors.is_empty() {
            errors.push("authors cannot be empty".to_owned());
        }
        if self.description.is_empty() {
            errors.push("description cannot be empty".to_owned());
        }
        if self.description.len() > 100 {
            errors.push("description must be less than 100 characters".to_owned());
        }
        if self.license.is_empty() {
            errors.push("license cannot be empty".to_owned());
        }
        // TODO: validate license
        for keyword in &self.keywords {
            const VALID_KEYWORDS: &[&str] = &["qol", "cheat", "bugfix", "cosmetic", "feature"];
            if !VALID_KEYWORDS.contains(&keyword.as_str()) {
                errors.push(format!("invalid keyword: {} (valid keywords: {:?})", keyword, VALID_KEYWORDS));
            }
        }
        errors
    }

    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    pub fn print_validation_warnings(&self) {
        for error in self.validate() {
            eprintln!("warning: {}", error);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String,
}

impl Config {
    pub fn read_from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut toml_string = String::new();
        reader.read_to_string(&mut toml_string)?;
        let config = toml::from_str(&toml_string)?;
        Ok(config)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let toml_string = toml::to_string_pretty(self)?;
        writer.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    pub fn default_for_mod(mod_path: &Path) -> Result<Self> {
        Ok(Self {
            package: Package {
                name: mod_path.file_name().unwrap().to_str().unwrap().to_owned(),
                version: "0.1.0".to_owned(),
                authors: vec![get_author()?],
                description: "An amazing mod".to_owned(),
                license: "CC-BY-SA-4.0".to_owned(),
                keywords: vec![],
            },
            base_commit: get_base_commit(mod_path)?,
            dependencies: HashMap::new(),
        })
    }
}

/// Get base commit of git submodule
fn get_base_commit(mod_path: &Path) -> Result<String> {
    let submodule_path = mod_path.join("papermario");
    let base_commit = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(&submodule_path)
        .output()?
        .stdout;
    let base_commit = String::from_utf8(base_commit)?;
    let base_commit = base_commit.trim().to_owned();
    Ok(base_commit)
}

/// Get author from git config as `name <email>`
fn get_author() -> Result<String> {
    let git_user_name = std::process::Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()?
        .stdout;
    let git_user_name = String::from_utf8(git_user_name)?;
    let git_user_name = git_user_name.trim().to_owned();
    let git_user_email = std::process::Command::new("git")
        .arg("config")
        .arg("user.email")
        .output()?
        .stdout;
    let git_user_email = String::from_utf8(git_user_email)?;
    let git_user_email = git_user_email.trim().to_owned();

    Ok(format!("{} <{}>", git_user_name, git_user_email))
}
