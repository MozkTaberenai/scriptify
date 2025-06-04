//! # Complex Pipes - Advanced Pipeline Operations
//!
//! This example demonstrates advanced pipeline operations with scriptify:
//! - Multi-branch pipelines
//! - Conditional pipeline execution
//! - Pipeline error handling
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete environment.rs

use scriptify::*;

fn main() -> Result<()> {
    println!("🔀 Complex Pipeline Operations");
    println!("============================\n");

    // 1. Multi-branch pipelines
    println!("1. Multi-branch pipelines:");
    multi_branch_pipelines()?;

    // 2. Conditional pipeline execution
    println!("\n2. Conditional pipeline execution:");
    conditional_pipelines()?;

    // 3. Pipeline error handling
    println!("\n3. Pipeline error handling:");
    pipeline_error_handling()?;

    println!("\n🎉 Complex pipeline tutorial completed!");
    Ok(())
}

fn multi_branch_pipelines() -> Result<()> {
    println!("🌳 Multi-branch pipeline operations:");

    // Create test data
    let test_data = "apple\nbanana\ncherry\ndate\nelderberry\nfig\ngrape";

    // Branch 1: Count items
    println!("\n📊 Branch 1: Count items");
    let count = cmd!("wc", "-l").input(test_data).output()?;
    println!("Total items: {}", count.trim());

    // Branch 2: Filter and process
    println!("\n🔍 Branch 2: Filter items starting with specific letters");
    let filtered_a_e = cmd!("grep", "^[ae]")
        .pipe(cmd!("sort"))
        .pipe(cmd!("tr", "\n", ", "))
        .input(test_data)
        .output()?;
    println!("Items starting with 'a' or 'e': {}", filtered_a_e.trim());

    // Branch 3: Transform and analyze
    println!("\n🔄 Branch 3: Transform and analyze");
    let lengths = cmd!("awk", "{print length($0), $0}")
        .pipe(cmd!("sort", "-n"))
        .input(test_data)
        .output()?;
    println!("Items sorted by length:");
    for line in lengths.lines() {
        if !line.trim().is_empty() {
            println!("  {}", line);
        }
    }

    Ok(())
}

fn conditional_pipelines() -> Result<()> {
    println!("🔀 Conditional pipeline execution:");

    // Create test data
    let test_data = "line1\nline2\nline3\nLINE4\nline5";
    let numbers_data = "10\n5\n20\n15\n8";

    // Conditional processing based on content
    println!("\n📊 Processing based on content:");

    // Check if data has uppercase content
    let has_uppercase = cmd!("grep", "[A-Z]").input(test_data).quiet().run().is_ok();

    if has_uppercase {
        println!("✅ Data contains uppercase - applying case normalization");
        let normalized = cmd!("tr", "[:upper:]", "[:lower:]")
            .pipe(cmd!("sort"))
            .input(test_data)
            .output()?;

        println!("Normalized content:");
        for line in normalized.lines() {
            if !line.trim().is_empty() {
                println!("  {}", line);
            }
        }
    } else {
        println!("ℹ️ No uppercase content found");
    }

    // Conditional numeric processing
    println!("\n🔢 Conditional numeric processing:");
    let max_number = cmd!("sort", "-n")
        .pipe(cmd!("tail", "-1"))
        .input(numbers_data)
        .output()?;

    let max_val: i32 = max_number.trim().parse().unwrap_or(0);

    if max_val > 15 {
        println!("📈 High values detected - applying processing");
        let processed = cmd!(
            "awk",
            "{sum+=$1} END {print \"Sum:\", sum, \"Count:\", NR, \"Average:\", sum/NR}"
        )
        .input(numbers_data)
        .output()?;

        println!("Processing result: {}", processed.trim());
    } else {
        println!("📊 Values within normal range");
    }

    Ok(())
}

fn pipeline_error_handling() -> Result<()> {
    println!("🛡️ Pipeline error handling and recovery:");

    // Pipeline with potential failure points
    println!("\n⚠️ Handling pipeline failures gracefully:");

    let test_data = "valid_line1\nvalid_line2\n\nvalid_line3";

    // Robust pipeline with error recovery
    match cmd!("grep", "-v", "^$") // Remove empty lines
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq"))
        .input(test_data)
        .output()
    {
        Ok(result) => {
            println!("✅ Pipeline succeeded:");
            for line in result.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => {
            println!("❌ Pipeline failed: {}", e);
            println!("🔄 Attempting recovery with simpler approach");

            // Fallback pipeline
            let fallback = cmd!("sort").input(test_data).output()?;
            println!("Fallback result:");
            for line in fallback.lines() {
                if !line.trim().is_empty() {
                    println!("  {}", line);
                }
            }
        }
    }

    // Pipeline with intermediate validation
    println!("\n🔍 Pipeline with intermediate validation:");
    let input_data = "1\n2\n3\ninvalid\n4\n5";

    // First stage: filter valid numbers
    let valid_numbers = cmd!("grep", "^[0-9]\\+$").input(input_data).output()?;

    if valid_numbers.trim().is_empty() {
        println!("❌ No valid numbers found");
        return Ok(());
    }

    println!("✅ Valid numbers found, continuing pipeline");

    // Second stage: process valid numbers
    let processed = cmd!("sort", "-n")
        .pipe(cmd!(
            "awk",
            "{sum+=$1} END {print \"Sum:\", sum, \"Count:\", NR, \"Average:\", sum/NR}"
        ))
        .input(&valid_numbers)
        .output()?;

    println!("Processing result: {}", processed.trim());

    Ok(())
}
