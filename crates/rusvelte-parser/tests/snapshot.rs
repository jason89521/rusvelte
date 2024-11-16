mod css;
mod simple;

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
                let result = parser.parse();
                match result {
                    Ok(root) => {
                        insta::assert_json_snapshot!(root);
                    }
                    Err(error) => {
                        eprintln!("Failed file: {}", path.to_string_lossy());
                        eprintln!("{}", error);

                        assert!(false);
                    }
                }
            });
        }
    };
}
