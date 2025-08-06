use std::path::PathBuf;

use core::traits::processor::DataProcessor;
use core::models::text::{TextContent, Filter};
use infrastructure::loaders::text_file_loader::TextFileLoader;
use service::analyzer::text_analyze_service::TextAnalyzer;

#[test]
fn test_end_to_end_text_analysis() {
    let loader = TextFileLoader;
    let analyzer = TextAnalyzer::new(Some(Filter::Contains("".to_string())));

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("tests/testdata/text_content.txt");

    let content = loader
        .process(file_path)
        .expect("Failed to load text file");

    let statistics = analyzer.process(&content).unwrap();

    assert_eq!(statistics.lines_count, 9);
    assert!(statistics.words_count > 10);
    assert!(statistics.unique_word_count > 5);
    assert!(statistics.average_word_length > 2.0);
}

#[test]
fn test_analysis_with_filter_only_matching_lines() {
    let content = TextContent::new(
        r#"
        Rust is great.
        I love systems programming.
        Rust makes safety easy.
        This line does not mention it.
        "#,
    );

    let filter = Filter::Contains("Rust".to_string());
    let analyzer = TextAnalyzer::new(Some(filter));

    let result = analyzer.process(&content).unwrap();

    // Erwartung: Nur 2 Zeilen enthalten "Rust"
    assert_eq!(result.lines_count, 2);
    assert!(result.words_count > 0);
    assert!(result.unique_word_count > 0);
}