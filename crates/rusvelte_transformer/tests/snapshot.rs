#[test]
fn test() {
    insta::glob!("samples/*.svelte", |path| {
        let source = std::fs::read_to_string(path).unwrap();
        let allocator = oxc_allocator::Allocator::default();
        let mut root = rusvelte_parser::Parser::new(&source, &allocator)
            .parse()
            .expect("Parse failed");
        let (scopes, _, symbols) = rusvelte_analyzer::Analyzer::default().analyze(&root);
        let program = rusvelte_transformer::Transformer::new(&allocator, scopes, symbols)
            .transform(&mut root);
        let code = oxc_codegen::Codegen::new().build(&program).code;
        insta::assert_snapshot!("client", code)
    })
}
