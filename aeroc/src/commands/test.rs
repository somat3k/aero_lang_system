use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn execute(path: PathBuf, filter: Option<String>) -> Result<()> {
    println!("Running tests...");

    // Verify Aero.toml exists
    let manifest_path = path.join("Aero.toml");
    if !manifest_path.exists() {
        anyhow::bail!(
            "Could not find Aero.toml in '{}'. Is this an AERO project?",
            path.display()
        );
    }

    // Find all .aero files in src/ and tests/
    let src_path = path.join("src");
    let tests_path = path.join("tests");

    let mut test_files = Vec::new();

    // Collect test files
    if src_path.exists() {
        for entry in walkdir::WalkDir::new(&src_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("aero"))
        {
            test_files.push(entry.path().to_path_buf());
        }
    }

    if tests_path.exists() {
        for entry in walkdir::WalkDir::new(&tests_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("aero"))
        {
            test_files.push(entry.path().to_path_buf());
        }
    }

    if test_files.is_empty() {
        println!("No test files found");
        return Ok(());
    }

    let mut _total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    for file_path in test_files {
        // Read source file
        let source = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        // Lex and parse to find test functions
        let tokens = aero_lexer::lex(&source)
            .with_context(|| format!("Lex error in {}", file_path.display()))?;

        let ast = aero_parser::parse(tokens)
            .with_context(|| format!("Parse error in {}", file_path.display()))?;

        // Find test functions (those with #[test] attribute)
        let test_functions = aero_ast::find_test_functions(&ast);

        for test_fn in test_functions {
            if let Some(ref f) = filter {
                if !test_fn.name.contains(f) {
                    continue;
                }
            }

            _total_tests += 1;
            print!("test {} ... ", test_fn.name);

            // In the future, we'll actually execute the test
            // For now, just check that it compiles
            match aero_types::check_function(&test_fn) {
                Ok(_) => {
                    println!("ok");
                    passed_tests += 1;
                }
                Err(e) => {
                    println!("FAILED");
                    eprintln!("  Error: {}", e);
                    failed_tests += 1;
                }
            }
        }
    }

    println!();
    println!("test result: {}. {} passed; {} failed",
        if failed_tests == 0 { "ok" } else { "FAILED" },
        passed_tests,
        failed_tests
    );

    if failed_tests > 0 {
        anyhow::bail!("{} test(s) failed", failed_tests);
    }

    Ok(())
}
