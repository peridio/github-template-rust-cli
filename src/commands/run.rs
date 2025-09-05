use crate::error::{Error, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: String,

    /// Optional output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Show statistics only (don't process the file)
    #[arg(long)]
    pub stats_only: bool,
}

pub fn execute(args: Args) -> Result<()> {
    info!("Processing file: {}", args.input);

    // Check if file exists
    if !Path::new(&args.input).exists() {
        warn!("File not found: {}", args.input);
        return Err(Error::Other(format!("File not found: {}", args.input)));
    }

    // Read and process file
    debug!("Reading file contents");
    let content = fs::read_to_string(&args.input)?;
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let byte_count = content.len();

    debug!(
        "File stats - lines: {}, words: {}, bytes: {}",
        line_count, word_count, byte_count
    );

    if args.stats_only {
        println!("File statistics for '{}':", args.input);
        println!("  Lines: {}", line_count);
        println!("  Words: {}", word_count);
        println!("  Bytes: {}", byte_count);
    } else {
        // Process the file (example: uppercase conversion)
        let processed = if let Some(output) = args.output {
            let uppercase_content = content.to_uppercase();
            fs::write(&output, uppercase_content)?;
            info!("Processed output written to: {}", output);
            println!("[SUCCESS] Output written to: {}", output);
            format!("Processed {} bytes to {}", byte_count, output)
        } else {
            // Just show stats if no output specified
            println!("File statistics:");
            println!("  Lines: {}", line_count);
            println!("  Words: {}", word_count);
            println!("  Bytes: {}", byte_count);
            format!("Analyzed {} bytes", byte_count)
        };

        info!("Processing complete: {}", processed);
        println!("[SUCCESS] Processing complete.");
    }

    Ok(())
}
