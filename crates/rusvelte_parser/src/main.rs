use std::fs;

use rusvelte_parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source =
        fs::read_to_string("input.svelte").map_err(|_| String::from("cannot read input.svelte"))?;
    let allocator = oxc_allocator::Allocator::default();
    let mut parser = Parser::new(&source, &allocator);
    // let result = parser.parse();
    match parser.parse() {
        Ok(result) => {
            let json = serde_json::to_string_pretty(&result)
                .map_err(|_| "Cannot convert to json".to_string())?;

            fs::write("output.json", json)
                .map_err(|_| "Cannot write to output.json".to_string())?;
        }
        Err(parser_error) => {
            eprintln!("remain: {}", parser.remain());
            eprintln!("{parser_error}");
        }
    }

    Ok(())
}
