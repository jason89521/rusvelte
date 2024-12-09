use oxc_ecmascript::BoundNames;
use rusvelte_ast::js_ast::{
    BindingRestElement, CatchParameter, Class, FormalParameter, Function, ImportDeclaration,
    VariableDeclarator,
};

use crate::binding::{BindingKind, DeclarationKind};

use super::scope_builder::ScopeBuilder;

pub trait Binder<'a> {
    #[allow(unused_variables)]
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind);
}

impl<'a> Binder<'a> for VariableDeclarator<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        self.id.bound_names(&mut |ident| {
            let symbol_id = builder.declare(&ident.name, kind, self.kind.into());
            ident.symbol_id.set(Some(symbol_id))
        });
    }
}

impl<'a> Binder<'a> for Function<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        if let Some(ident) = &self.id {
            let symbol_id = builder.declare(&ident.name, kind, DeclarationKind::Function);
            ident.set_symbol_id(symbol_id);
        }
    }
}

impl<'a> Binder<'a> for FormalParameter<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        self.pattern.bound_names(&mut |ident| {
            let symbol_id = builder.declare(&ident.name, kind, DeclarationKind::Param);
            ident.symbol_id.set(Some(symbol_id));
        });
    }
}

impl<'a> Binder<'a> for BindingRestElement<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        self.bound_names(&mut |ident| {
            let symbol_id = builder.declare(&ident.name, kind, DeclarationKind::RestParam);
            ident.symbol_id.set(Some(symbol_id));
        });
    }
}

impl<'a> Binder<'a> for ImportDeclaration<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        if let Some(specifiers) = &self.specifiers {
            for specifier in specifiers.iter() {
                let symbol_id = builder.declare(&specifier.name(), kind, DeclarationKind::Import);
                specifier.local().set_symbol_id(symbol_id);
            }
        }
    }
}

impl<'a> Binder<'a> for CatchParameter<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        self.pattern.bound_names(&mut |ident| {
            let symbol_id = builder.declare(&ident.name, kind, DeclarationKind::Let);
            ident.symbol_id.set(Some(symbol_id));
        });
    }
}

impl<'a> Binder<'a> for Class<'a> {
    fn bind(&self, builder: &mut ScopeBuilder<'a>, kind: BindingKind) {
        let Some(ident) = &self.id else { return };
        let symbol_id = builder.declare(&ident.name, kind, DeclarationKind::Let);
        ident.symbol_id.set(Some(symbol_id));
    }
}
