use oxc_ast::ast::{
    ArrayAssignmentTarget, AssignmentTarget, AssignmentTargetMaybeDefault,
    AssignmentTargetProperty, AssignmentTargetPropertyIdentifier, AssignmentTargetPropertyProperty,
    AssignmentTargetRest, AssignmentTargetWithDefault, ComputedMemberExpression, Expression,
    IdentifierReference, ObjectAssignmentTarget, PrivateFieldExpression, SimpleAssignmentTarget,
    StaticMemberExpression,
};

use super::extract_identifier::ExtractIdentifier;

pub enum UnwrapPatternItem<'a> {
    IdentifierReference(&'a IdentifierReference<'a>),
    StaticMemberExpression(&'a StaticMemberExpression<'a>),
    ComputedMemberExpression(&'a ComputedMemberExpression<'a>),
    PrivateFieldExpression(&'a PrivateFieldExpression<'a>),
}

impl UnwrapPatternItem<'_> {
    pub fn is_identifier_reference(&self) -> bool {
        matches!(self, UnwrapPatternItem::IdentifierReference(_))
    }
}

impl<'a> ExtractIdentifier<'a> for UnwrapPatternItem<'a> {
    fn extract_identifier(&self) -> Option<&IdentifierReference<'a>> {
        match self {
            UnwrapPatternItem::IdentifierReference(x) => Some(x),
            UnwrapPatternItem::StaticMemberExpression(x) => x.extract_identifier(),
            UnwrapPatternItem::ComputedMemberExpression(x) => x.extract_identifier(),
            UnwrapPatternItem::PrivateFieldExpression(x) => x.extract_identifier(),
        }
    }
}

pub trait UnwrapPattern<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        vec![]
    }
}

impl<'a> UnwrapPattern<'a> for IdentifierReference<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        vec![UnwrapPatternItem::IdentifierReference(self)]
    }
}

impl<'a> UnwrapPattern<'a> for Expression<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        match self {
            Expression::Identifier(x) => x.unwrap_pattern(),
            Expression::StaticMemberExpression(x) => x.unwrap_pattern(),
            Expression::ComputedMemberExpression(x) => x.unwrap_pattern(),
            Expression::PrivateFieldExpression(x) => x.unwrap_pattern(),
            _ => vec![],
        }
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTarget<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        match self {
            AssignmentTarget::AssignmentTargetIdentifier(x) => x.unwrap_pattern(),
            AssignmentTarget::ComputedMemberExpression(x) => x.unwrap_pattern(),
            AssignmentTarget::StaticMemberExpression(x) => x.unwrap_pattern(),
            AssignmentTarget::PrivateFieldExpression(x) => x.unwrap_pattern(),
            AssignmentTarget::ArrayAssignmentTarget(x) => x.unwrap_pattern(),
            AssignmentTarget::ObjectAssignmentTarget(x) => x.unwrap_pattern(),
            _ => vec![],
        }
    }
}

impl<'a> UnwrapPattern<'a> for StaticMemberExpression<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        vec![UnwrapPatternItem::StaticMemberExpression(self)]
    }
}

impl<'a> UnwrapPattern<'a> for ComputedMemberExpression<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        vec![UnwrapPatternItem::ComputedMemberExpression(self)]
    }
}

impl<'a> UnwrapPattern<'a> for PrivateFieldExpression<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        vec![UnwrapPatternItem::PrivateFieldExpression(self)]
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetRest<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        self.target.unwrap_pattern()
    }
}

impl<'a> UnwrapPattern<'a> for ArrayAssignmentTarget<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        let mut result: Vec<UnwrapPatternItem<'a>> = self
            .elements
            .iter()
            .flat_map(|element| {
                if let Some(element) = element {
                    element.unwrap_pattern()
                } else {
                    vec![]
                }
            })
            .collect();
        if let Some(rest) = self.rest.as_ref() {
            result.extend(rest.unwrap_pattern());
        }

        result
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetWithDefault<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        self.binding.unwrap_pattern()
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetPropertyIdentifier<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        self.binding.unwrap_pattern()
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetPropertyProperty<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        self.binding.unwrap_pattern()
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetProperty<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        match self {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(x) => x.unwrap_pattern(),
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(x) => x.unwrap_pattern(),
        }
    }
}

impl<'a> UnwrapPattern<'a> for ObjectAssignmentTarget<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        let mut result: Vec<UnwrapPatternItem<'a>> = self
            .properties
            .iter()
            .flat_map(|property| property.unwrap_pattern())
            .collect();
        if let Some(rest) = self.rest.as_ref() {
            result.extend(rest.unwrap_pattern());
        }

        result
    }
}

impl<'a> UnwrapPattern<'a> for AssignmentTargetMaybeDefault<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        match self {
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::ComputedMemberExpression(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::StaticMemberExpression(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::PrivateFieldExpression(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::ArrayAssignmentTarget(x) => x.unwrap_pattern(),
            AssignmentTargetMaybeDefault::ObjectAssignmentTarget(x) => x.unwrap_pattern(),
            _ => vec![],
        }
    }
}

impl<'a> UnwrapPattern<'a> for SimpleAssignmentTarget<'a> {
    fn unwrap_pattern(&'a self) -> Vec<UnwrapPatternItem<'a>> {
        match self {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(x) => x.unwrap_pattern(),
            SimpleAssignmentTarget::ComputedMemberExpression(x) => x.unwrap_pattern(),
            SimpleAssignmentTarget::StaticMemberExpression(x) => x.unwrap_pattern(),
            SimpleAssignmentTarget::PrivateFieldExpression(x) => x.unwrap_pattern(),
            _ => vec![],
        }
    }
}
