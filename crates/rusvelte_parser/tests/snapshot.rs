mod attributes;
mod block;
mod css;
mod element;
mod script;
mod simple;
mod tag;

#[macro_export]
macro_rules! test_success {
    () => {
        #[test]
        fn test_success() {
            use rusvelte_parser::Parser;
            insta::glob!("inputs/*.svelte", |path| {
                let source = std::fs::read_to_string(path).unwrap();
                let allocator = oxc_allocator::Allocator::default();
                let mut parser = Parser::new(&source, &allocator);
                let ret = parser.parse();
                if !ret.errors.is_empty() {
                    eprintln!("Failed file: {}", path.to_string_lossy());
                    eprintln!("{:?}", ret.errors);

                    assert!(false);
                }
                insta::assert_json_snapshot!(&ret.root);
            });
        }
    };
}
