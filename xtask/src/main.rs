use clap::{Parser, Subcommand};
use scriptify::*;
use std::path::PathBuf;

mod readme;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Development task runner for scriptify
#[derive(Parser)]
#[command(
    name = "xtask",
    about = "Development task runner for scriptify",
    long_about = "⚠️  IMPORTANT: README.md is auto-generated from src/lib.rs\n   To update README.md: edit src/lib.rs and run 'cargo xtask readme'\n\nBefore committing: cargo xtask ci",
    version
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress output (overrides verbose)
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate README.md from lib.rs documentation
    Readme {
        /// Force regeneration even if README.md is newer than lib.rs
        #[arg(short, long)]
        force: bool,
    },
    /// Generate and open documentation
    Docs {
        /// Generate docs without opening browser
        #[arg(long)]
        no_open: bool,
        /// Include private items in documentation
        #[arg(long)]
        document_private_items: bool,
    },
    /// Run all tests
    Test {
        /// Run only tests matching this pattern
        #[arg(long)]
        filter: Option<String>,
        /// Show output from successful tests
        #[arg(long)]
        nocapture: bool,
    },
    /// Run cargo check
    Check {
        /// Check all targets (including tests and examples)
        #[arg(long)]
        all_targets: bool,
    },
    /// Format code
    Fmt {
        /// Check if code is formatted without making changes
        #[arg(long)]
        check: bool,
    },
    /// Run clippy lints
    Clippy {
        /// Treat warnings as errors
        #[arg(short, long)]
        deny_warnings: bool,
        /// Fix clippy suggestions automatically
        #[arg(long)]
        fix: bool,
    },
    /// Clean build artifacts
    Clean {
        /// Also clean documentation
        #[arg(long)]
        doc: bool,
    },
    /// Run benchmarks
    Bench {
        /// Benchmark filter pattern
        #[arg(long)]
        filter: Option<String>,
    },
    /// Generate test coverage report
    Coverage {
        /// Output format (html, xml, json)
        #[arg(long, default_value = "html")]
        format: String,
        /// Open coverage report after generation
        #[arg(long)]
        open: bool,
    },
    /// Run pre-commit checks (test + clippy + fmt)
    Precommit,
    /// Run all CI tasks
    Ci,
}

fn get_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if current_name == "xtask" {
        Ok(current_dir.parent().unwrap().to_path_buf())
    } else {
        Ok(current_dir)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set global verbosity
    let verbose = cli.verbose && !cli.quiet;
    let _quiet = cli.quiet;

    match cli.command {
        Commands::Readme { force } => generate_readme(force)?,
        Commands::Docs {
            no_open,
            document_private_items,
        } => generate_docs(no_open, document_private_items)?,
        Commands::Test { filter, nocapture } => run_tests(filter, nocapture, verbose)?,
        Commands::Check { all_targets } => run_check(all_targets, verbose)?,
        Commands::Fmt { check } => run_fmt(check, verbose)?,
        Commands::Clippy { deny_warnings, fix } => run_clippy(deny_warnings, fix, verbose)?,
        Commands::Clean { doc } => run_clean(doc, verbose)?,
        Commands::Bench { filter } => run_bench(filter, verbose)?,
        Commands::Coverage { format, open } => run_coverage(format, open, verbose)?,
        Commands::Precommit => run_precommit(verbose)?,
        Commands::Ci => run_ci(verbose)?,
    }

    Ok(())
}

fn generate_readme(force: bool) -> Result<()> {
    readme::generate_readme_with_options(force)
}

fn run_tests(filter: Option<String>, nocapture: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🧪 Running tests...");
    }
    let project_root = get_project_root()?;

    let mut cmd = cmd!("cargo", "test").cwd(&project_root);

    if let Some(pattern) = filter {
        cmd = cmd.arg(pattern);
    }

    if nocapture {
        cmd = cmd.arg("--").arg("--nocapture");
    }

    if verbose {
        cmd = cmd.arg("--verbose");
    }

    cmd.run()?;

    if !verbose {
        echo!("✅ Tests passed!");
    }
    Ok(())
}

fn generate_docs(no_open: bool, document_private_items: bool) -> Result<()> {
    echo!("📚 Generating documentation...");
    let project_root = get_project_root()?;

    let mut cmd = cmd!("cargo", "doc", "--no-deps").cwd(&project_root);

    if !no_open {
        cmd = cmd.arg("--open");
    }

    if document_private_items {
        cmd = cmd.arg("--document-private-items");
    }

    cmd.run()?;

    if no_open {
        echo!("✅ Documentation generated!");
    } else {
        echo!("✅ Documentation generated and opened!");
    }
    Ok(())
}

fn run_check(all_targets: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🔍 Running cargo check...");
    }
    let project_root = get_project_root()?;

    let mut cmd = cmd!("cargo", "check").cwd(&project_root);

    if all_targets {
        cmd = cmd.arg("--all-targets");
    }

    if verbose {
        cmd = cmd.arg("--verbose");
    }

    cmd.run()?;

    if !verbose {
        echo!("✅ Check passed!");
    }
    Ok(())
}

fn run_fmt(check_only: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🎨 Formatting code...");
    }
    let project_root = get_project_root()?;

    if check_only {
        let result = cmd!("cargo", "fmt", "--", "--check")
            .cwd(&project_root)
            .quiet()
            .run();

        if result.is_err() {
            echo!("❌ Code is not properly formatted!");
            echo!("Run 'cargo xtask fmt' to fix formatting");
            std::process::exit(1);
        } else if !verbose {
            echo!("✅ Code is properly formatted!");
        }
    } else {
        // Check if formatting is needed first
        let fmt_check = cmd!("cargo", "fmt", "--", "--check")
            .cwd(&project_root)
            .quiet()
            .run();

        if fmt_check.is_err() {
            if !verbose {
                echo!("Code needs formatting, applying changes...");
            }
            cmd!("cargo", "fmt").cwd(&project_root).run()?;
            if !verbose {
                echo!("✅ Code formatted!");
            }
        } else if !verbose {
            echo!("✅ Code is already properly formatted!");
        }
    }
    Ok(())
}

fn run_clippy(deny_warnings: bool, fix: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("📎 Running comprehensive clippy checks...");
    }
    let project_root = get_project_root()?;

    let warning_flag = if deny_warnings { "-D" } else { "-W" };

    if !verbose {
        echo!("  Running clippy for all targets...");
    }
    let mut cmd = cmd!("cargo", "clippy", "--all-targets", "--all-features").cwd(&project_root);

    if fix {
        cmd = cmd.arg("--fix").arg("--allow-dirty");
    }

    cmd = cmd.arg("--").arg(warning_flag).arg("warnings");
    cmd.run()?;

    if !verbose {
        echo!("  Running clippy for tests...");
    }
    let mut test_cmd = cmd!("cargo", "clippy", "--tests").cwd(&project_root);

    if fix {
        test_cmd = test_cmd.arg("--fix").arg("--allow-dirty");
    }

    test_cmd = test_cmd.arg("--").arg(warning_flag).arg("warnings");
    test_cmd.run()?;

    if !verbose {
        echo!("  Running clippy for examples...");
    }
    let mut example_cmd = cmd!("cargo", "clippy", "--examples").cwd(&project_root);

    if fix {
        example_cmd = example_cmd.arg("--fix").arg("--allow-dirty");
    }

    example_cmd = example_cmd.arg("--").arg(warning_flag).arg("warnings");
    example_cmd.run()?;

    if !verbose {
        echo!("✅ All clippy checks passed!");
    }
    Ok(())
}

fn run_clean(doc: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🧹 Cleaning build artifacts...");
    }
    let project_root = get_project_root()?;

    cmd!("cargo", "clean").cwd(&project_root).run()?;

    if doc {
        cmd!("cargo", "clean", "--doc").cwd(&project_root).run()?;
    }

    if !verbose {
        echo!("✅ Build artifacts cleaned!");
    }
    Ok(())
}

fn run_bench(filter: Option<String>, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🏃 Running benchmarks...");
    }

    let project_root = get_project_root()?;

    // Check if benches directory exists
    if project_root.join("benches").exists() {
        let mut cmd = cmd!("cargo", "bench").cwd(&project_root);

        if let Some(pattern) = filter {
            cmd = cmd.arg(pattern);
        }

        cmd.run()?;

        if !verbose {
            echo!("✅ Benchmarks completed!");
        }
    } else if !verbose {
        echo!("⚠️  No benchmarks found (benches/ directory not present)");
    }
    Ok(())
}

fn run_coverage(format: String, open: bool, verbose: bool) -> Result<()> {
    if !verbose {
        echo!("📊 Generating test coverage report...");
    }

    // Check if cargo-tarpaulin is installed
    let tarpaulin_check = cmd!("cargo", "tarpaulin", "--version").quiet().run();

    if tarpaulin_check.is_ok() {
        if !verbose {
            echo!("Using cargo-tarpaulin for coverage...");
        }
        let project_root = get_project_root()?;

        let format_arg = match format.as_str() {
            "html" => "Html",
            "xml" => "Xml",
            "json" => "Json",
            _ => "Html",
        };

        cmd!(
            "cargo",
            "tarpaulin",
            "--out",
            format_arg,
            "--output-dir",
            "target/coverage"
        )
        .cwd(&project_root)
        .run()?;

        if !verbose {
            echo!("✅ Coverage report generated in target/coverage/");
        }

        if open && format == "html" {
            let coverage_file = project_root.join("target/coverage/tarpaulin-report.html");
            if coverage_file.exists() {
                #[cfg(target_os = "macos")]
                cmd!("open", coverage_file.to_string_lossy()).run()?;
                #[cfg(target_os = "linux")]
                cmd!("xdg-open", coverage_file.to_string_lossy()).run()?;
                #[cfg(target_os = "windows")]
                cmd!("start", coverage_file.to_string_lossy()).run()?;
            }
        }
    } else {
        echo!("⚠️  cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin");
        echo!("Falling back to basic test run...");
        run_tests(None, false, verbose)?;
    }
    Ok(())
}

fn run_precommit(verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🔍 Running pre-commit checks...");
    }

    // Run tests first
    run_tests(None, false, verbose)?;

    // Run comprehensive clippy
    run_clippy(true, false, verbose)?;

    // Format code
    run_fmt(false, verbose)?;

    if !verbose {
        echo!("🎉 Pre-commit checks completed successfully!");
        echo!("✅ Ready to commit!");
    }

    Ok(())
}

fn run_ci(verbose: bool) -> Result<()> {
    if !verbose {
        echo!("🚀 Running full CI pipeline...");
    }

    // Check formatting first
    run_fmt(false, verbose)?;

    // Run static analysis
    run_clippy(true, false, verbose)?;

    // Check compilation
    run_check(true, verbose)?;

    // Run tests
    run_tests(None, false, verbose)?;

    // Generate documentation
    generate_readme(false)?;

    if !verbose {
        echo!("🎉 All CI tasks completed successfully!");
        echo!("🔍 Summary:");
        echo!("  ✅ Code formatting");
        echo!("  ✅ Clippy lints");
        echo!("  ✅ Compilation check");
        echo!("  ✅ Test suite");
        echo!("  ✅ README generation");
    }

    Ok(())
}
