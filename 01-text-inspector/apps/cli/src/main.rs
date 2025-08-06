use std::path::PathBuf;
use clap::Parser;

use core::models::text::Content;
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_loader = TextFileLoader;
    let loader_service = Service::new(&file_loader);
    let analyzer = TextAnalyzer;

    let content: Content = loader_service.load_content(args.file)?;

    let stats = analyzer.process(&content)?;

    println!("Lines: {}", stats.lines_count);
    println!("Words: {}", stats.words_count);
    println!("Unique Words: {}", stats.unique_word_count);
    println!("Avg Word Length: {:.2}", stats.average_word_length);

    Ok(())
}
