use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn execute(path: PathBuf, release: bool) -> Result<()> {
    let mode = if release { "release" } else { "debug" };
    println!("Building AERO project in {} mode...", mode);

    // Verify Aero.toml exists
    let manifest_path = path.join("Aero.toml");
    if !manifest_path.exists() {
        anyhow::bail!(
            "Could not find Aero.toml in '{}'. Is this an AERO project?",
            path.display()
        );
    }

    // Read and parse manifest
    let manifest_content = std::fs::read_to_string(&manifest_path)
        .context("Failed to read Aero.toml")?;

    let manifest: toml::Value = toml::from_str(&manifest_content)
        .context("Failed to parse Aero.toml")?;

    let package_name = manifest
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing package.name in Aero.toml"))?;

    // Create target directory
    let target_dir = path.join("target").join(mode);
    std::fs::create_dir_all(&target_dir)
        .context("Failed to create target directory")?;

    // Find all .aero files in src/
    let src_path = path.join("src");
    if !src_path.exists() {
        anyhow::bail!("Could not find src/ directory");
    }

    let mut compiled_files = 0;

    for entry in walkdir::WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("aero"))
    {
        let file_path = entry.path();
        print!("  Compiling {}...", file_path.display());

        // Read source file
        let source = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        // Lex
        let tokens = aero_lexer::lex(&source)
            .with_context(|| format!("Lex error in {}", file_path.display()))?;

        // Parse
        let ast = aero_parser::parse(tokens)
            .with_context(|| format!("Parse error in {}", file_path.display()))?;

        // Type check
        let typed_ast = aero_types::check(&ast)
            .with_context(|| format!("Type error in {}", file_path.display()))?;

        // Code generation
        let bytecode = aero_codegen::generate(&typed_ast, release)
            .with_context(|| format!("Code generation error in {}", file_path.display()))?;

        println!(" ✓");
        compiled_files += 1;

        // For now, just keep the bytecode in memory
        // In the future, we'll link all modules together
        let _ = bytecode;
    }

    // Generate final .avm bytecode file
    let output_path = target_dir.join(format!("{}.avm", package_name));

    // For now, create a placeholder bytecode file
    // In the future, this will be the linked bytecode from all modules
    let bytecode = avm_runtime::Bytecode::new();
    bytecode.write_to_file(&output_path)
        .context("Failed to write bytecode file")?;

    println!();
    println!("✓ Compiled {} file(s)", compiled_files);
    println!("   Output: {}", output_path.display());

    Ok(())
}
