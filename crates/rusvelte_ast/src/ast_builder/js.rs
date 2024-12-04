use super::AstBuilder;

use oxc_allocator::{Box, IntoIn, Vec};
use oxc_ast::{ast::*, NONE};
use oxc_span::{Atom, SPAN};

impl<'a> AstBuilder<'a> {
    pub fn statement_import_declaration(self, decl: ImportDeclaration<'a>) -> Statement<'a> {
        Statement::ImportDeclaration(self.builder.alloc(decl))
    }

    pub fn import_all(self, r#as: &str, source: &str) -> ImportDeclaration<'a> {
        let local = self.builder.binding_identifier(SPAN, r#as);
        let specifier = self
            .builder
            .import_declaration_specifier_import_namespace_specifier(SPAN, local);
        let source = self.builder.string_literal(SPAN, source, None);
        self.builder.import_declaration(
            SPAN,
            Some(self.vec([specifier])),
            source,
            NONE,
            ImportOrExportKind::Value,
        )
    }

    pub fn call_with_atom<A>(self, callee: A, args: Vec<Argument<'a>>) -> CallExpression<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        let callee = self.builder.expression_identifier_reference(SPAN, callee);
        let args = self.vec_from_iter(args);
        self.builder
            .call_expression(SPAN, callee, NONE, args, false)
    }

    pub fn expression_call_with_atom<A>(self, callee: A, args: Vec<Argument<'a>>) -> Expression<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        Expression::CallExpression(self.alloc(self.call_with_atom(callee, args)))
    }

    pub fn move_expression(self, expr: &mut Expression<'a>) -> Expression<'a> {
        self.builder.move_expression(expr)
    }

    pub fn expression_identifier_reference(self, name: &str) -> Expression<'a> {
        self.builder.expression_identifier_reference(SPAN, name)
    }

    pub fn move_function(self, function: &mut Function<'a>) -> Function<'a> {
        self.builder.move_function(function)
    }

    pub fn move_declaration(self, decl: &mut Declaration<'a>) -> Declaration<'a> {
        self.builder.move_declaration(decl)
    }

    pub fn move_statement(self, stmt: &mut Statement<'a>) -> Statement<'a> {
        self.builder.move_statement(stmt)
    }

    pub fn alloc<T>(self, value: T) -> Box<'a, T> {
        self.builder.alloc(value)
    }

    pub fn statement_expression(self, expr: Expression<'a>) -> Statement<'a> {
        self.builder.statement_expression(SPAN, expr)
    }

    pub fn var(self, id: BindingPattern<'a>, init: Expression<'a>) -> VariableDeclaration<'a> {
        let kind = VariableDeclarationKind::Var;
        let decl = self
            .builder
            .variable_declarator(SPAN, kind, id, Some(init), false);
        self.builder
            .variable_declaration(SPAN, kind, self.vec([decl]), false)
    }

    pub fn statement_var(self, id: BindingPattern<'a>, init: Expression<'a>) -> Statement<'a> {
        Statement::VariableDeclaration(self.alloc(self.var(id, init)))
    }

    pub fn binding_pattern_identifier<A>(self, name: A) -> BindingPattern<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        let kind = self
            .builder
            .binding_pattern_kind_binding_identifier(SPAN, name);
        self.builder.binding_pattern(kind, NONE, false)
    }

    pub fn function_declaration<A>(
        self,
        name: A,
        params: Vec<'a, FormalParameter<'a>>,
        statements: Vec<'a, Statement<'a>>,
    ) -> Function<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        let params = self.builder.formal_parameters(
            SPAN,
            FormalParameterKind::FormalParameter,
            params,
            NONE,
        );
        let body = self.builder.function_body(SPAN, self.vec([]), statements);
        self.builder.function(
            FunctionType::FunctionDeclaration,
            SPAN,
            Some(self.builder.binding_identifier(SPAN, name)),
            false,
            false,
            false,
            NONE,
            NONE,
            params,
            NONE,
            Some(body),
        )
    }

    pub fn arrow(
        self,
        params: Vec<'a, FormalParameter<'a>>,
        statements: Vec<'a, Statement<'a>>,
    ) -> ArrowFunctionExpression<'a> {
        let params = self.builder.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            params,
            NONE,
        );
        let expression =
            statements.len() == 1 && matches!(statements[0], Statement::ExpressionStatement(_));
        let body = self.builder.function_body(SPAN, self.vec([]), statements);

        self.builder
            .arrow_function_expression(SPAN, expression, false, NONE, params, NONE, body)
    }

    pub fn expression_arrow(
        self,
        params: Vec<'a, FormalParameter<'a>>,
        statements: Vec<'a, Statement<'a>>,
    ) -> Expression<'a> {
        Expression::ArrowFunctionExpression(self.alloc(self.arrow(params, statements)))
    }

    pub fn formal_parameter(self, pattern: BindingPattern<'a>) -> FormalParameter<'a> {
        self.builder
            .formal_parameter(SPAN, self.vec([]), pattern, None, false, false)
    }

    pub fn statement_export_default_function_declaration(
        self,
        function: Function<'a>,
    ) -> Statement<'a> {
        let declaration = ExportDefaultDeclarationKind::FunctionDeclaration(self.alloc(function));
        let export = self.builder.export_default_declaration(
            SPAN,
            declaration,
            self.builder
                .module_export_name_identifier_name(SPAN, "default"),
        );
        Statement::ExportDefaultDeclaration(self.alloc(export))
    }

    pub fn string_literal<A>(self, value: A) -> StringLiteral<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        self.builder.string_literal(SPAN, value, None)
    }

    pub fn statement_import_declaration_without_specifier<A>(self, source: A) -> Statement<'a>
    where
        A: IntoIn<'a, Atom<'a>>,
    {
        let decl = self.builder.alloc_import_declaration(
            SPAN,
            None,
            self.string_literal(source),
            NONE,
            ImportOrExportKind::Value,
        );
        Statement::ImportDeclaration(decl)
    }
}
