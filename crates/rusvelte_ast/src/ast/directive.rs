use oxc_ast::ast::Expression;
use oxc_span::Span;
use rusvelte_derive::{AstTree, OxcSpan};

use super::attribute::AttributeValue;

#[derive(Debug, AstTree, OxcSpan)]
pub enum Directive<'a> {
    AnimateDirective(AnimateDirective<'a>),
    BindDirective(BindDirective<'a>),
    ClassDirective(ClassDirective<'a>),
    LetDirective(LetDirective<'a>),
    OnDirective(OnDirective<'a>),
    StyleDirective(StyleDirective<'a>),
    TransitionDirective(TransitionDirective<'a>),
    UseDirective(UseDirective<'a>),
}

impl<'a> Directive<'a> {
    /// # Panics
    ///
    /// Panics if `expression` is none when trying to construct a ClassDirective or UseDirective.
    pub fn new(
        span: Span,
        kind: DirectiveKind<'a>,
        name: &'a str,
        expression: Option<Expression<'a>>,
        modifiers: Vec<&'a str>,
    ) -> Self {
        let directive = match kind {
            DirectiveKind::AnimateDirective => Directive::AnimateDirective(AnimateDirective {
                span,
                name,
                expression,
            }),
            DirectiveKind::BindDirective => Directive::BindDirective(BindDirective {
                span,
                name,
                expression,
            }),
            DirectiveKind::ClassDirective => {
                if let Some(expression) = expression {
                    Directive::ClassDirective(ClassDirective {
                        span,
                        name,
                        expression,
                    })
                } else {
                    panic!("Trying to construct a ClassDirective without expression")
                }
            }
            DirectiveKind::LetDirective => Directive::LetDirective(LetDirective {
                span,
                name,
                expression,
            }),
            DirectiveKind::OnDirective => Directive::OnDirective(OnDirective {
                span,
                name,
                expression,
                modifiers,
            }),
            DirectiveKind::StyleDirective => {
                panic!("Should not use this method to create a StyleDirective")
            }
            DirectiveKind::TransitionDirective(direction) => {
                let (intro, outro) = if direction == "transition" {
                    (true, true)
                } else {
                    (direction == "in", direction == "out")
                };
                Directive::TransitionDirective(TransitionDirective {
                    span,
                    name,
                    expression,
                    modifiers,
                    intro,
                    outro,
                })
            }
            DirectiveKind::UseDirective => {
                if let Some(expression) = expression {
                    Directive::UseDirective(UseDirective {
                        span,
                        name,
                        expression,
                    })
                } else {
                    panic!("Trying to construct a UseDirective without expression")
                }
            }
        };

        directive
    }

    /// Only useful when Directive is [TransitionDirective]
    pub fn set_direction(&mut self, direction: &'a str) {
        if let Directive::TransitionDirective(directive) = self {
            directive.intro = direction == "in" || direction == "direction"
        }
    }

    pub fn expression(&self) -> Option<&Expression<'a>> {
        match self {
            Directive::AnimateDirective(x) => x.expression.as_ref(),
            Directive::BindDirective(x) => x.expression.as_ref(),
            Directive::ClassDirective(x) => Some(&x.expression),
            Directive::LetDirective(x) => x.expression.as_ref(),
            Directive::OnDirective(x) => x.expression.as_ref(),
            Directive::StyleDirective(_) => None,
            Directive::TransitionDirective(x) => x.expression.as_ref(),
            Directive::UseDirective(x) => Some(&x.expression),
        }
    }

    pub fn set_expression(&mut self, expression: Expression<'a>) {
        match self {
            Directive::AnimateDirective(x) => x.expression = Some(expression),
            Directive::BindDirective(x) => x.expression = Some(expression),
            Directive::ClassDirective(x) => x.expression = expression,
            Directive::LetDirective(x) => x.expression = Some(expression),
            Directive::OnDirective(x) => x.expression = Some(expression),
            Directive::StyleDirective(_) => (),
            Directive::TransitionDirective(x) => x.expression = Some(expression),
            Directive::UseDirective(x) => x.expression = expression,
        }
    }

    pub fn name(&self) -> &'a str {
        match self {
            Directive::AnimateDirective(x) => x.name,
            Directive::BindDirective(x) => x.name,
            Directive::ClassDirective(x) => x.name,
            Directive::LetDirective(x) => x.name,
            Directive::OnDirective(x) => x.name,
            Directive::StyleDirective(x) => x.name,
            Directive::TransitionDirective(x) => x.name,
            Directive::UseDirective(x) => x.name,
        }
    }

    pub fn kind_str(&self) -> &'static str {
        match self {
            Directive::AnimateDirective(_) => "AnimateDirective",
            Directive::BindDirective(_) => "BindDirective",
            Directive::ClassDirective(_) => "ClassDirective",
            Directive::LetDirective(_) => "LetDirective",
            Directive::OnDirective(_) => "OnDirective",
            Directive::StyleDirective(_) => "StyleDirective",
            Directive::TransitionDirective(_) => "TransitionDirective",
            Directive::UseDirective(_) => "UseDirective",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DirectiveKind<'a> {
    AnimateDirective,
    BindDirective,
    ClassDirective,
    LetDirective,
    OnDirective,
    StyleDirective,
    TransitionDirective(&'a str),
    UseDirective,
}

impl<'a> DirectiveKind<'a> {
    pub fn from_name(name: &'a str) -> Option<DirectiveKind<'a>> {
        let kind = match name {
            "use" => DirectiveKind::UseDirective,
            "animate" => DirectiveKind::AnimateDirective,
            "bind" => DirectiveKind::BindDirective,
            "class" => DirectiveKind::ClassDirective,
            "style" => DirectiveKind::StyleDirective,
            "on" => DirectiveKind::OnDirective,
            "let" => DirectiveKind::LetDirective,
            "in" | "out" | "transition" => DirectiveKind::TransitionDirective(name),
            _ => return None,
        };
        Some(kind)
    }
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct AnimateDirective<'a> {
    pub span: Span,
    /// The `x` in `animate:x`
    pub name: &'a str,
    /// The `y` in `animate:x={y}`
    pub expression: Option<Expression<'a>>,
}
#[derive(Debug, AstTree, OxcSpan)]
pub struct BindDirective<'a> {
    pub span: Span,
    /// The `x` in `bind:x`
    pub name: &'a str,
    /// The `y` in `bind:x={y}`
    pub expression: Option<Expression<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct ClassDirective<'a> {
    pub span: Span,
    /// The `y` in `class:x={y}`, or the `x` in `class:x`
    pub name: &'a str,
    /// The `y`` in `class:x={y}`, or the `x` in `class:x`
    pub expression: Expression<'a>,
}
#[derive(Debug, AstTree, OxcSpan)]
pub struct LetDirective<'a> {
    pub span: Span,
    /// The `x` in `let:x`
    pub name: &'a str,
    /// The `y` in `let:x={y}`
    pub expression: Option<Expression<'a>>,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct OnDirective<'a> {
    pub span: Span,
    /// The `x` in `on:x`
    pub name: &'a str,
    /// The `y` in `on:x={y}`
    pub expression: Option<Expression<'a>>,
    pub modifiers: Vec<&'a str>,
}
#[derive(Debug, AstTree, OxcSpan)]
pub struct StyleDirective<'a> {
    pub span: Span,
    /// The `x` in `style:x`
    pub name: &'a str,
    /// The `y` in `style:x={y}`
    pub value: AttributeValue<'a>,
    // TODO: should be `important`
    pub modifiers: Vec<&'a str>,
}
#[derive(Debug, AstTree, OxcSpan)]
pub struct TransitionDirective<'a> {
    pub span: Span,
    /// The `x` in `transition:x`
    pub name: &'a str,
    /// The `y` in `transition:x={y}`
    pub expression: Option<Expression<'a>>,
    // TODO: should be `local` | `global`
    pub modifiers: Vec<&'a str>,
    pub intro: bool,
    pub outro: bool,
}

#[derive(Debug, AstTree, OxcSpan)]
pub struct UseDirective<'a> {
    pub span: Span,
    /// The `x` in `use:x`
    pub name: &'a str,
    /// The `y` in `use:x={y}`
    pub expression: Expression<'a>,
}
