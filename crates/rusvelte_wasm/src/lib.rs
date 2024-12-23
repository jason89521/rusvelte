use oxc_allocator::Allocator;
use rusvelte_parser::Parser;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct Rusvelte {
    #[wasm_bindgen(readonly)]
    pub ast: JsValue,
}

#[wasm_bindgen]
impl Rusvelte {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen]
    pub fn parse(&mut self, source: &str) {
        let allocator = Allocator::default();
        let mut parser = Parser::new(source, &allocator);

        let root = parser.parse().root;

        self.ast = root
            .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap();
    }
}
