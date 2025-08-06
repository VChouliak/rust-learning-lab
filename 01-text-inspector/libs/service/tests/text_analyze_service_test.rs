use std::path::PathBuf;

use core::traits::processor::DataProcessor;
use infrastructure::loaders::text_file_loader::TextFileLoader;
use service::analyzer::text_analyze_service::TextAnalyzer;

#[test]
fn test_end_to_end_text_analysis() {
    let loader = TextFileLoader;
    let analyzer = TextAnalyzer;

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
