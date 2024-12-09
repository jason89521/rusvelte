use oxc_ast::ast::{
    ComputedMemberExpression, Expression, IdentifierReference, MemberExpression,
    PrivateFieldExpression, StaticMemberExpression,
};

pub trait ExtractIdentifier<'a> {
    /// Gets the left-most identifier of a member expression or identifier.
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        None
    }
}

impl<'a> ExtractIdentifier<'a> for IdentifierReference<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        Some(self)
    }
}

impl<'a> ExtractIdentifier<'a> for Expression<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        match self {
            Expression::Identifier(x) => x.extract_identifier(),
            Expression::StaticMemberExpression(x) => x.extract_identifier(),
            Expression::ComputedMemberExpression(x) => x.extract_identifier(),
            Expression::PrivateFieldExpression(x) => x.extract_identifier(),
            _ => None,
        }
    }
}

impl<'a> ExtractIdentifier<'a> for StaticMemberExpression<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        self.object.extract_identifier()
    }
}

impl<'a> ExtractIdentifier<'a> for ComputedMemberExpression<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        self.object.extract_identifier()
    }
}

impl<'a> ExtractIdentifier<'a> for PrivateFieldExpression<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        self.object.extract_identifier()
    }
}

impl<'a> ExtractIdentifier<'a> for MemberExpression<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        match self {
            MemberExpression::ComputedMemberExpression(x) => x.extract_identifier(),
            MemberExpression::StaticMemberExpression(x) => x.extract_identifier(),
            MemberExpression::PrivateFieldExpression(x) => x.extract_identifier(),
        }
    }
}
