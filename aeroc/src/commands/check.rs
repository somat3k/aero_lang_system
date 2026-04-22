use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn execute(path: PathBuf) -> Result<()> {
    println!("Checking AERO project at '{}'...", path.display());

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

    let _manifest: toml::Value = toml::from_str(&manifest_content)
        .context("Failed to parse Aero.toml")?;

    // Find all .aero files in src/
    let src_path = path.join("src");
    if !src_path.exists() {
        anyhow::bail!("Could not find src/ directory");
    }

    let mut checked_files = 0;
    let mut errors = 0;

    for entry in walkdir::WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("aero"))
    {
        let file_path = entry.path();
        print!("  Checking {}...", file_path.display());

        // Read source file
        let source = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        // Run lexer
        match aero_lexer::lex(&source) {
            Ok(tokens) => {
                // Run parser
                match aero_parser::parse(tokens) {
                    Ok(ast) => {
                        // Run type checker
                        match aero_types::check(&ast) {
                            Ok(_) => {
                                println!(" ✓");
                                checked_files += 1;
                            }
                            Err(e) => {
                                println!(" ✗");
                                eprintln!("Type error in {}: {}", file_path.display(), e);
                                errors += 1;
                            }
                        }
                    }
                    Err(e) => {
                        println!(" ✗");
                        eprintln!("Parse error in {}: {}", file_path.display(), e);
                        errors += 1;
                    }
                }
            }
            Err(e) => {
                println!(" ✗");
                eprintln!("Lex error in {}: {}", file_path.display(), e);
                errors += 1;
            }
        }
    }

    println!();
    if errors == 0 {
        println!("✓ Checked {} file(s) - no errors found", checked_files);
        Ok(())
    } else {
        anyhow::bail!("Found {} error(s) in {} file(s)", errors, checked_files);
    }
}
