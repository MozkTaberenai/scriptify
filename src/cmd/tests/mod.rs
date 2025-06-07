//! Tests for the `cmd` module.
//!
//! This module contains comprehensive tests for the command execution functionality,
//! organized into separate modules by category for better maintainability.

// Re-export items needed by test modules
use super::*;

// Test modules
mod basic;
mod concurrency;
mod environment;
mod error_handling;
mod input_output;

mod no_echo;
mod pipeline;
mod quoting;
mod security;
