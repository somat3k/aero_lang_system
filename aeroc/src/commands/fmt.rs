use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn execute(path: PathBuf, check: bool) -> Result<()> {
    if check {
        println!("Checking formatting...");
    } else {
        println!("Formatting AERO source files...");
    }

    // Find all .aero files in src/
    let src_path = path.join("src");
    if !src_path.exists() {
        anyhow::bail!("Could not find src/ directory");
    }

    let mut formatted_files = 0;
    let mut needs_formatting = 0;

    for entry in walkdir::WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("aero"))
    {
        let file_path = entry.path();

        // Read source file
        let source = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        // Lex and parse
        let tokens = aero_lexer::lex(&source)
            .with_context(|| format!("Lex error in {}", file_path.display()))?;

        let ast = aero_parser::parse(tokens)
            .with_context(|| format!("Parse error in {}", file_path.display()))?;

        // Format the AST — propagates an error if any construct is unsupported
        let formatted = aero_ast::format(&ast)
            .with_context(|| format!("Format error in {}", file_path.display()))?;

        if source != formatted {
            needs_formatting += 1;

            if check {
                println!("  {} needs formatting", file_path.display());
            } else {
                std::fs::write(file_path, formatted)
                    .with_context(|| format!("Failed to write {}", file_path.display()))?;
                println!("  Formatted {}", file_path.display());
                formatted_files += 1;
            }
        }
    }

    println!();
    if check {
        if needs_formatting == 0 {
            println!("✓ All files are formatted correctly");
            Ok(())
        } else {
            anyhow::bail!("{} file(s) need formatting", needs_formatting);
        }
    } else {
        if formatted_files == 0 {
            println!("✓ All files already formatted");
        } else {
            println!("✓ Formatted {} file(s)", formatted_files);
        }
        Ok(())
    }
}
