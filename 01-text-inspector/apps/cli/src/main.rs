use std::path::PathBuf;
use clap::builder::Str;
use clap::Parser;

use core::models::text::{TextContent, Filter};
use core::traits::processor::DataProcessor;
use infrastructure::loaders::text_file_loader::TextFileLoader;
use service::loader::text_loader_service::Service;
use service::analyzer::text_analyze_service::TextAnalyzer;

/// Simple CLI tool to analyze text files.
#[derive(Parser, Debug)]
#[command(name = "Text Inspector", version, author, about = "Analyze text files easily")]
struct Args {
    /// Path to the text file to analyze
    #[arg(short, long)]
    file: PathBuf,
    /// Filter lines containing this string
    #[arg(long)]
    contains: Option<String>,

    /// Filter lines starting with this string
    #[arg(long)]
    starts_with: Option<String>,

    /// Filter lines ending with this string
    #[arg(long)]
    ends_with: Option<String>,

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create filter based on provided arguments
    let filter = if let Some(term) = args.contains {
        Some(Filter::Contains(term))
    } else if let Some(prefix) = args.starts_with {
        Some(Filter::StartsWith(prefix))
    } else if let Some(suffix) = args.ends_with {
        Some(Filter::EndsWith(suffix))
    } else {
        None
    };

    let file_loader = TextFileLoader;
    let loader_service = Service::new(&file_loader);
    let analyzer = TextAnalyzer::new(filter);

    let content: TextContent = loader_service.load_content(args.file)?;

    let stats = analyzer.process(&content)?;

    println!("Lines: {}", stats.lines_count);
    println!("Words: {}", stats.words_count);
    println!("Unique Words: {}", stats.unique_word_count);
    println!("Avg Word Length: {:.2}", stats.average_word_length);

    Ok(())
}
