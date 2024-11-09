use std::fs;

use svelte_parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source =
        fs::read_to_string("input.svelte").map_err(|_| String::from("cannot read input.svelte"))?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    let result = parser.parse()?;

    let json =
        serde_json::to_string_pretty(&result).map_err(|_| format!("Cannot convert to json"))?;

    fs::write("output.json", json).map_err(|_| format!("Cannot write to output.json"))?;

    Ok(())
}
