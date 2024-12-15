use rusvelte_ast::ast::ExpressionMetadata;

#[derive(Debug, Default)]
pub struct State {
    expression: Option<ExpressionMetadata>,
}

impl State {
    pub fn replace_expression_metadata(
        &mut self,
        expression: Option<ExpressionMetadata>,
    ) -> Option<ExpressionMetadata> {
        if let Some(expression) = expression {
            self.expression.replace(expression)
        } else {
            self.expression.take()
        }
    }

    pub fn expression_metadata(&self) -> Option<&ExpressionMetadata> {
        self.expression.as_ref()
    }
}
