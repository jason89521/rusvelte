use oxc_ecmascript::BoundNames;
use rusvelte_ast::js_ast::{Function, FunctionType, VariableDeclarator};

use crate::{
    symbol::{BindingKind, DeclarationKind},
    Analyzer,
};

pub trait Binder<'a> {
    #[allow(unused_variables)]
    fn bind(&self, analyzer: &mut Analyzer<'a>, kind: BindingKind) {}
}

impl<'a> Binder<'a> for VariableDeclarator<'a> {
    fn bind(&self, analyzer: &mut Analyzer<'a>, kind: BindingKind) {
        if self.kind.is_lexical() {
            self.id.bound_names(&mut |ident| {
                let symbol_id =
                    analyzer.declare_symbol(ident.span, &ident.name, kind, self.kind.into());
                ident.symbol_id.set(Some(symbol_id))
            });
        }
    }
}

impl<'a> Binder<'a> for Function<'a> {
    fn bind(&self, analyzer: &mut Analyzer<'a>, kind: BindingKind) {
        if let Some(ident) = &self.id {
            if self.r#type == FunctionType::FunctionDeclaration {
                let symbol_id = analyzer.declare_symbol(
                    ident.span,
                    &ident.name,
                    kind,
                    DeclarationKind::Function,
                );
                ident.set_symbol_id(symbol_id);
            }
        }
    }
}
