use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

const AERO_TOML_TEMPLATE: &str = r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2026"

[capabilities]
# Uncomment to enable network capabilities
# network = ["outbound:https"]

[dependencies]
# Add dependencies here
# aero-http = "1.2"

# [world.adapter_name]
# adapter = "AdapterType"
# url = "config"
# poll_interval = "5s"
"#;

const MAIN_AERO_TEMPLATE: &str = r#"// Welcome to AERO!
// This is your program's entry point.

fn main() ! [log] {
    emit log::info("Hello, AERO world!");

    // The 'know' keyword asserts knowledge instead of binding
    know greeting = "Welcome to AERO";
    emit log::info(greeting);
}
"#;

const GITIGNORE_TEMPLATE: &str = r#"# AERO build artifacts
/target/
*.avm
Aero.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
"#;

pub fn execute(name: String, path: Option<PathBuf>) -> Result<()> {
    let project_path = match path {
        Some(p) => p.join(&name),
        None => PathBuf::from(&name),
    };

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_path.display());
    }

    // Create project directory structure
    fs::create_dir_all(&project_path)
        .context("Failed to create project directory")?;

    let src_path = project_path.join("src");
    fs::create_dir(&src_path)
        .context("Failed to create src directory")?;

    // Generate Aero.toml
    let aero_toml = AERO_TOML_TEMPLATE.replace("{name}", &name);
    fs::write(project_path.join("Aero.toml"), aero_toml)
        .context("Failed to write Aero.toml")?;

    // Generate src/main.aero
    fs::write(src_path.join("main.aero"), MAIN_AERO_TEMPLATE)
        .context("Failed to write main.aero")?;

    // Generate .gitignore
    fs::write(project_path.join(".gitignore"), GITIGNORE_TEMPLATE)
        .context("Failed to write .gitignore")?;

    println!("     Created AERO project '{}'", name);
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  aeroc run");
    println!();
    println!("To learn more, visit: https://docs.aero-lang.dev");

    Ok(())
}
