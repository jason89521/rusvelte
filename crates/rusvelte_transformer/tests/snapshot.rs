use rusvelte_analyzer::{Analysis, Analyzer, CompileOptions};

#[test]
fn test() {
    insta::glob!("samples/**/input.svelte", |path| {
        let folder_path = std::path::Path::new(path).parent().unwrap();
        let source = std::fs::read_to_string(path).unwrap();
        let allocator = oxc_allocator::Allocator::default();
        let mut root = rusvelte_parser::Parser::new(&source, &allocator)
            .parse()
            .expect("Parse failed");
        let analyzer = Analyzer::new(CompileOptions::new("App".to_string()), &root);
        let Analysis {
            scopes,
            symbols,
            references,
            ..
        } = analyzer.analyze(&root);
        let program =
            rusvelte_transformer::Transformer::new(&allocator, scopes, symbols, references)
                .client_transform(&mut root);
        let code = oxc_codegen::Codegen::new().build(&program).code;
        insta::with_settings!({snapshot_path => folder_path, snapshot_suffix => "", prepend_module_to_snapshot => false}, {
            insta::assert_snapshot!("client", code)
        })
    })
}
