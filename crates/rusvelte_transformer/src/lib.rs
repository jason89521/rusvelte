use std::cell::Cell;

use oxc_allocator::{Allocator, Vec as OxcVec};
use oxc_span::{SourceType, SPAN};
use rusvelte_analyzer::{
    binding::{Binding, BindingTable},
    reference::ReferenceTable,
    scope::Scopes,
    ScopeId, SymbolId,
};
use rusvelte_ast::{
    ast::Root,
    ast_builder::AstBuilder,
    js_ast::{Program, Statement},
    visit::JsVisitMut,
};

mod js;
mod svelte;

struct TransformState<'a> {
    allocator: &'a Allocator,
    update: OxcVec<'a, Statement<'a>>,
}

impl<'a> TransformState<'a> {
    fn new(allocator: &'a Allocator) -> Self {
        Self {
            allocator,
            update: OxcVec::new_in(allocator),
        }
    }

    fn take_update(&mut self) -> OxcVec<'a, Statement<'a>> {
        std::mem::replace(&mut self.update, OxcVec::new_in(self.allocator))
    }
}

pub struct Transformer<'a> {
    ast: AstBuilder<'a>,
    allocator: &'a Allocator,
    hoisted: OxcVec<'a, Statement<'a>>,
    scopes: Scopes,
    symbols: BindingTable,
    reference_table: ReferenceTable,
    current_scope_id: ScopeId,
    state: TransformState<'a>,
}

impl<'a> Transformer<'a> {
    pub fn new(
        allocator: &'a Allocator,
        scopes: Scopes,
        symbols: BindingTable,
        reference_table: ReferenceTable,
    ) -> Self {
        let ast = AstBuilder::new(allocator);
        let hoisted = ast
            .vec([ast.statement_import_declaration(ast.import_all("$", "svelte/internal/client"))]);
        let current_scope_id = scopes.root_scope_id();

        Self {
            allocator,
            hoisted,
            ast,
            scopes,
            symbols,
            reference_table,
            current_scope_id,
            state: TransformState::new(allocator),
        }
    }

    pub fn client_transform(mut self, root: &mut Root<'a>) -> Program<'a> {
        let mut instance_body = self.ast.vec([]);
        if let Some(script) = root.instance.as_mut() {
            self.visit_program(&mut script.content);
            instance_body = std::mem::replace(&mut script.content.body, instance_body);
        }

        let template_body = self.visit_fragment(&mut root.fragment);

        let component_block = OxcVec::from_iter_in(
            instance_body.into_iter().chain(template_body),
            self.allocator,
        );
        let component = self.ast.function_declaration(
            "App",
            self.ast.vec([self
                .ast
                .formal_parameter(self.ast.binding_pattern_identifier("$$anchor"))]),
            component_block,
        );

        let mut body = self.ast.vec([]);
        // TODO: check option.discloseVersion
        body.push(
            self.ast
                .statement_import_declaration_without_specifier("svelte/internal/disclose-version"),
        );
        body.append(&mut self.hoisted);
        // TODO: check option.hmr
        body.push(
            self.ast
                .statement_export_default_function_declaration(component),
        );

        Program {
            span: SPAN,
            source_type: SourceType::mjs(),
            source_text: "",
            comments: OxcVec::new_in(self.allocator),
            hashbang: None,
            directives: OxcVec::new_in(self.allocator),
            body,
            scope_id: Cell::new(None),
        }
    }

    #[allow(unused)]
    fn find_binding_mut(&mut self, name: &str) -> Option<(SymbolId, &mut Binding)> {
        self.scopes
            .find_symbol_id(self.current_scope_id, name)
            .map(|symbol_id| (symbol_id, self.symbols.get_binding_mut(symbol_id)))
    }

    fn find_binding(&mut self, name: &str) -> Option<(SymbolId, &Binding)> {
        self.scopes
            .find_symbol_id(self.current_scope_id, name)
            .map(|symbol_id| (symbol_id, self.symbols.get_binding(symbol_id)))
    }
}
