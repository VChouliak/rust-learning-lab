use std::path::PathBuf;
use core::traits::processor::DataProcessor;
use infrastructure::loaders::text_file_loader::TextFileLoader;

#[test]
fn test_text_file_loader_reads_file_correctly() {
    let loader = TextFileLoader;
    
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("tests/testdata/text_content.txt");

    let result = loader.process(file_path);

    assert!(result.is_ok());

    let content = result.unwrap();
    assert!(content.content.contains("Rust"));
    assert_eq!(content.content.lines().count(), 9);
}
