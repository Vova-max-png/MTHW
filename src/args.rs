use clap::Parser;

/// Program to read docs
#[derive(Parser, Debug)]
#[command(author="Cat", version, about, long_about = None)]
pub struct Args {
    /// Path to file
    pub path: String,
}