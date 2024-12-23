use std::fs;

use oxc_codegen::Codegen;
use rusvelte_analyzer::{Analysis, Analyzer, CompileOptions};
use rusvelte_parser::Parser;
use rusvelte_transformer::Transformer;

#[allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("input.svelte")?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    let mut root = parser.parse().root;
    let analyzer = Analyzer::new(CompileOptions::new("App".to_string()), &root);
    let Analysis {
        scopes,
        symbols,
        references,
        ..
    } = analyzer.analyze(&root);

    let transformer = Transformer::new(&allocator, scopes, symbols, references);
    let program = transformer.client_transform(&mut root);

    let instance = Codegen::new().build(&program);
    println!("{}", instance.code);

    Ok(())
}
