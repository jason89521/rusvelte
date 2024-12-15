use std::fs;

use rusvelte_analyzer::{Analyzer, CompileOptions};
use rusvelte_parser::Parser;

#[allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("input.svelte")?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    let root = parser.parse()?;

    let analyzer = Analyzer::new(CompileOptions::new("App".to_string()), &root);
    let analysis = analyzer.analyze(&root);

    // println!("scopes: {:?}", analyzer.scopes);
    // println!("symbols: {:#?}", analyzer.symbols);

    Ok(())
}
