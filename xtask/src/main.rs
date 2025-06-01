use scriptify::*;
use std::env;
use std::path::PathBuf;

mod readme;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_name = current_dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    
    if current_name == "xtask" {
        Ok(current_dir.parent().unwrap().to_path_buf())
    } else {
        Ok(current_dir)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        echo!("Usage: cargo xtask <task>");
        echo!("Available tasks:");
        echo!("  readme    - Generate README.md from lib.rs documentation");
        echo!("  docs      - Generate and open documentation");
        echo!("  test      - Run all tests");
        echo!("  check     - Run cargo check");
        echo!("  fmt       - Format code");
        echo!("  clippy    - Run clippy lints");
        echo!("  clean     - Clean build artifacts");
        echo!("  bench     - Run benchmarks");
        echo!("  coverage  - Generate test coverage report");
        echo!("  ci        - Run all CI tasks");
        return Ok(());
    }

    let task = &args[1];
    
    match task.as_str() {
        "readme" => generate_readme()?,
        "docs" => generate_docs()?,
        "test" => run_tests()?,
        "check" => run_check()?,
        "fmt" => run_fmt()?,
        "clippy" => run_clippy()?,
        "clean" => run_clean()?,
        "bench" => run_bench()?,
        "coverage" => run_coverage()?,
        "ci" => run_ci()?,
        _ => {
            echo!("Unknown task:", task);
            echo!("Run 'cargo xtask' for available tasks");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn generate_readme() -> Result<()> {
    readme::generate_readme()
}

fn run_tests() -> Result<()> {
    echo!("🧪 Running tests...");
    let project_root = get_project_root()?;
    cmd!("cargo", "test").cwd(&project_root).run()?;
    echo!("✅ Tests passed!");
    Ok(())
}

fn generate_docs() -> Result<()> {
    echo!("📚 Generating documentation...");
    let project_root = get_project_root()?;
    cmd!("cargo", "doc", "--open", "--no-deps").cwd(&project_root).run()?;
    echo!("✅ Documentation generated and opened!");
    Ok(())
}

fn run_check() -> Result<()> {
    echo!("🔍 Running cargo check...");
    let project_root = get_project_root()?;
    cmd!("cargo", "check", "--all-targets").cwd(&project_root).run()?;
    echo!("✅ Check passed!");
    Ok(())
}

fn run_fmt() -> Result<()> {
    echo!("🎨 Formatting code...");
    let project_root = get_project_root()?;
    
    // Check if formatting is needed first
    let fmt_check = cmd!("cargo", "fmt", "--", "--check").cwd(&project_root).quiet().run();
    
    if fmt_check.is_err() {
        echo!("Code needs formatting, applying changes...");
        cmd!("cargo", "fmt").cwd(&project_root).run()?;
        echo!("✅ Code formatted!");
    } else {
        echo!("✅ Code is already properly formatted!");
    }
    Ok(())
}

fn run_clippy() -> Result<()> {
    echo!("📎 Running clippy...");
    let project_root = get_project_root()?;
    cmd!("cargo", "clippy", "--all-targets", "--", "-D", "warnings").cwd(&project_root).run()?;
    echo!("✅ Clippy passed!");
    Ok(())
}

fn run_clean() -> Result<()> {
    echo!("🧹 Cleaning build artifacts...");
    let project_root = get_project_root()?;
    cmd!("cargo", "clean").cwd(&project_root).run()?;
    echo!("✅ Build artifacts cleaned!");
    Ok(())
}

fn run_bench() -> Result<()> {
    echo!("🏃 Running benchmarks...");
    
    let project_root = get_project_root()?;
    
    // Check if benches directory exists
    if project_root.join("benches").exists() {
        cmd!("cargo", "bench").cwd(&project_root).run()?;
        echo!("✅ Benchmarks completed!");
    } else {
        echo!("⚠️  No benchmarks found (benches/ directory not present)");
    }
    Ok(())
}

fn run_coverage() -> Result<()> {
    echo!("📊 Generating test coverage report...");
    
    // Check if cargo-tarpaulin is installed
    let tarpaulin_check = cmd!("cargo", "tarpaulin", "--version").quiet().run();
    
    if tarpaulin_check.is_ok() {
        echo!("Using cargo-tarpaulin for coverage...");
        let project_root = get_project_root()?;
        cmd!("cargo", "tarpaulin", "--out", "Html", "--output-dir", "target/coverage")
            .cwd(&project_root)
            .run()?;
        echo!("✅ Coverage report generated in target/coverage/");
    } else {
        echo!("⚠️  cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin");
        echo!("Falling back to basic test run...");
        run_tests()?;
    }
    Ok(())
}

fn run_ci() -> Result<()> {
    echo!("🚀 Running full CI pipeline...");
    
    // Check formatting first
    run_fmt()?;
    
    // Run static analysis
    run_clippy()?;
    
    // Check compilation
    run_check()?;
    
    // Run tests
    run_tests()?;
    
    // Generate documentation
    generate_readme()?;
    
    echo!("🎉 All CI tasks completed successfully!");
    echo!("🔍 Summary:");
    echo!("  ✅ Code formatting");
    echo!("  ✅ Clippy lints");
    echo!("  ✅ Compilation check");
    echo!("  ✅ Test suite");
    echo!("  ✅ README generation");
    
    Ok(())
}