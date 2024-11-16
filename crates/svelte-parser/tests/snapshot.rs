mod css;
mod simple;

#[macro_export]
macro_rules! test_success {
    () => {
        #[test]
        fn test_success() {
            use svelte_parser::Parser;
            insta::glob!("inputs/*.svelte", |path| {
                let source = std::fs::read_to_string(path).unwrap();
                let allocator = oxc_allocator::Allocator::default();
                let mut parser = Parser::new(&source, &allocator);
                let result = parser.parse();
                assert!(result.is_ok());
                let root = result.unwrap();
                insta::assert_json_snapshot!(root)
            });
        }
    };
}
