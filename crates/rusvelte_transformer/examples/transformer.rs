use std::fs;

use oxc_codegen::Codegen;
use rusvelte_analyzer::Analyzer;
use rusvelte_parser::Parser;
use rusvelte_transformer::Transformer;

#[allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("input.svelte")?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    let mut root = parser.parse()?;
    let analyzer = Analyzer::default();
    let (scopes, nodes, symbols, reference_table) = analyzer.analyze(&root);

    let transformer = Transformer::new(&allocator, scopes, symbols, reference_table);
    let program = transformer.client_transform(&mut root);

    let instance = Codegen::new().build(&program);
    println!("{}", instance.code);

    Ok(())
}
