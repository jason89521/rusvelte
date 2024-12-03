use std::fs;

use oxc_syntax::node::NodeId;
use rusvelte_analyzer::Analyzer;
use rusvelte_parser::Parser;

#[allow(unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("input.svelte")?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    let root = parser.parse()?;

    let analyzer = Analyzer::default();
    let (scopes, nodes, symbols) = analyzer.analyze(&root);

    // println!("scopes: {:?}", analyzer.scopes);
    // println!("symbols: {:#?}", analyzer.symbols);
    println!("{:#?}", nodes.get_node(NodeId::new(23)));

    Ok(())
}
