use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn execute(path: PathBuf, release: bool, args: Vec<String>) -> Result<()> {
    let mode = if release { "release" } else { "debug" };

    // First, build the project
    println!("Building and running AERO project...");
    super::build::execute(path.clone(), release)?;

    // Read manifest to get package name
    let manifest_path = path.join("Aero.toml");
    let manifest_content = std::fs::read_to_string(&manifest_path)
        .context("Failed to read Aero.toml")?;

    let manifest: toml::Value = toml::from_str(&manifest_content)
        .context("Failed to parse Aero.toml")?;

    let package_name = manifest
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing package.name in Aero.toml"))?;

    // Find the bytecode file
    let bytecode_path = path.join("target").join(mode).join(format!("{}.avm", package_name));
    if !bytecode_path.exists() {
        anyhow::bail!("Bytecode file not found: {}", bytecode_path.display());
    }

    println!();
    println!("Running {}...", package_name);
    println!("─────────────────────────────────────");

    // Load and execute the bytecode
    let bytecode = avm_runtime::Bytecode::load_from_file(&bytecode_path)
        .context("Failed to load bytecode")?;

    let mut vm = avm_runtime::VM::new();
    vm.execute(&bytecode, &args)
        .context("Runtime error")?;

    Ok(())
}
