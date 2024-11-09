use oxc_ast::ast::*;
use oxc_ast::visit::walk_mut::*;
use oxc_ast::VisitMut;
use oxc_syntax::scope::ScopeFlags;

pub struct SpanOffset(pub u32);

impl<'a> VisitMut<'a> for SpanOffset {
    #[inline]
    fn visit_program(&mut self, it: &mut Program<'a>) {
        walk_program(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_hashbang(&mut self, it: &mut Hashbang<'a>) {
        walk_hashbang(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_directive(&mut self, it: &mut Directive<'a>) {
        walk_directive(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_string_literal(&mut self, it: &mut StringLiteral<'a>) {
        walk_string_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_block_statement(&mut self, it: &mut BlockStatement<'a>) {
        walk_block_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_break_statement(&mut self, it: &mut BreakStatement<'a>) {
        walk_break_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_label_identifier(&mut self, it: &mut LabelIdentifier<'a>) {
        walk_label_identifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_continue_statement(&mut self, it: &mut ContinueStatement<'a>) {
        walk_continue_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_debugger_statement(&mut self, it: &mut DebuggerStatement) {
        walk_debugger_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_do_while_statement(&mut self, it: &mut DoWhileStatement<'a>) {
        walk_do_while_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_boolean_literal(&mut self, it: &mut BooleanLiteral) {
        walk_boolean_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_null_literal(&mut self, it: &mut NullLiteral) {
        walk_null_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_numeric_literal(&mut self, it: &mut NumericLiteral<'a>) {
        walk_numeric_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_big_int_literal(&mut self, it: &mut BigIntLiteral<'a>) {
        walk_big_int_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_reg_exp_literal(&mut self, it: &mut RegExpLiteral<'a>) {
        walk_reg_exp_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_template_literal(&mut self, it: &mut TemplateLiteral<'a>) {
        walk_template_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_template_element(&mut self, it: &mut TemplateElement<'a>) {
        walk_template_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_identifier_reference(&mut self, it: &mut IdentifierReference<'a>) {
        walk_identifier_reference(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_meta_property(&mut self, it: &mut MetaProperty<'a>) {
        walk_meta_property(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_identifier_name(&mut self, it: &mut IdentifierName<'a>) {
        walk_identifier_name(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_super(&mut self, it: &mut Super) {
        walk_super(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_array_expression(&mut self, it: &mut ArrayExpression<'a>) {
        walk_array_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_spread_element(&mut self, it: &mut SpreadElement<'a>) {
        walk_spread_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_elision(&mut self, it: &mut Elision) {
        walk_elision(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_arrow_function_expression(&mut self, it: &mut ArrowFunctionExpression<'a>) {
        walk_arrow_function_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_parameter_declaration(&mut self, it: &mut TSTypeParameterDeclaration<'a>) {
        walk_ts_type_parameter_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_parameter(&mut self, it: &mut TSTypeParameter<'a>) {
        walk_ts_type_parameter(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_binding_identifier(&mut self, it: &mut BindingIdentifier<'a>) {
        walk_binding_identifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_any_keyword(&mut self, it: &mut TSAnyKeyword) {
        walk_ts_any_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_big_int_keyword(&mut self, it: &mut TSBigIntKeyword) {
        walk_ts_big_int_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_boolean_keyword(&mut self, it: &mut TSBooleanKeyword) {
        walk_ts_boolean_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_intrinsic_keyword(&mut self, it: &mut TSIntrinsicKeyword) {
        walk_ts_intrinsic_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_never_keyword(&mut self, it: &mut TSNeverKeyword) {
        walk_ts_never_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_null_keyword(&mut self, it: &mut TSNullKeyword) {
        walk_ts_null_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_number_keyword(&mut self, it: &mut TSNumberKeyword) {
        walk_ts_number_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_object_keyword(&mut self, it: &mut TSObjectKeyword) {
        walk_ts_object_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_string_keyword(&mut self, it: &mut TSStringKeyword) {
        walk_ts_string_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_symbol_keyword(&mut self, it: &mut TSSymbolKeyword) {
        walk_ts_symbol_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_undefined_keyword(&mut self, it: &mut TSUndefinedKeyword) {
        walk_ts_undefined_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_unknown_keyword(&mut self, it: &mut TSUnknownKeyword) {
        walk_ts_unknown_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_void_keyword(&mut self, it: &mut TSVoidKeyword) {
        walk_ts_void_keyword(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_array_type(&mut self, it: &mut TSArrayType<'a>) {
        walk_ts_array_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_conditional_type(&mut self, it: &mut TSConditionalType<'a>) {
        walk_ts_conditional_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_constructor_type(&mut self, it: &mut TSConstructorType<'a>) {
        walk_ts_constructor_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_formal_parameters(&mut self, it: &mut FormalParameters<'a>) {
        walk_formal_parameters(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_formal_parameter(&mut self, it: &mut FormalParameter<'a>) {
        walk_formal_parameter(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_decorator(&mut self, it: &mut Decorator<'a>) {
        walk_decorator(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_object_pattern(&mut self, it: &mut ObjectPattern<'a>) {
        walk_object_pattern(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_binding_property(&mut self, it: &mut BindingProperty<'a>) {
        walk_binding_property(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_private_identifier(&mut self, it: &mut PrivateIdentifier<'a>) {
        walk_private_identifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_binding_rest_element(&mut self, it: &mut BindingRestElement<'a>) {
        walk_binding_rest_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_array_pattern(&mut self, it: &mut ArrayPattern<'a>) {
        walk_array_pattern(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_pattern(&mut self, it: &mut AssignmentPattern<'a>) {
        walk_assignment_pattern(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_annotation(&mut self, it: &mut TSTypeAnnotation<'a>) {
        walk_ts_type_annotation(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_function_type(&mut self, it: &mut TSFunctionType<'a>) {
        walk_ts_function_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_this_parameter(&mut self, it: &mut TSThisParameter<'a>) {
        walk_ts_this_parameter(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_import_type(&mut self, it: &mut TSImportType<'a>) {
        walk_ts_import_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_qualified_name(&mut self, it: &mut TSQualifiedName<'a>) {
        walk_ts_qualified_name(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_import_attributes(&mut self, it: &mut TSImportAttributes<'a>) {
        walk_ts_import_attributes(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_import_attribute(&mut self, it: &mut TSImportAttribute<'a>) {
        walk_ts_import_attribute(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_parameter_instantiation(&mut self, it: &mut TSTypeParameterInstantiation<'a>) {
        walk_ts_type_parameter_instantiation(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_indexed_access_type(&mut self, it: &mut TSIndexedAccessType<'a>) {
        walk_ts_indexed_access_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_infer_type(&mut self, it: &mut TSInferType<'a>) {
        walk_ts_infer_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_intersection_type(&mut self, it: &mut TSIntersectionType<'a>) {
        walk_ts_intersection_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_literal_type(&mut self, it: &mut TSLiteralType<'a>) {
        walk_ts_literal_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_unary_expression(&mut self, it: &mut UnaryExpression<'a>) {
        walk_unary_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_mapped_type(&mut self, it: &mut TSMappedType<'a>) {
        walk_ts_mapped_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_named_tuple_member(&mut self, it: &mut TSNamedTupleMember<'a>) {
        walk_ts_named_tuple_member(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_optional_type(&mut self, it: &mut TSOptionalType<'a>) {
        walk_ts_optional_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_rest_type(&mut self, it: &mut TSRestType<'a>) {
        walk_ts_rest_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_template_literal_type(&mut self, it: &mut TSTemplateLiteralType<'a>) {
        walk_ts_template_literal_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_this_type(&mut self, it: &mut TSThisType) {
        walk_ts_this_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_tuple_type(&mut self, it: &mut TSTupleType<'a>) {
        walk_ts_tuple_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_literal(&mut self, it: &mut TSTypeLiteral<'a>) {
        walk_ts_type_literal(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_index_signature(&mut self, it: &mut TSIndexSignature<'a>) {
        walk_ts_index_signature(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_index_signature_name(&mut self, it: &mut TSIndexSignatureName<'a>) {
        walk_ts_index_signature_name(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_property_signature(&mut self, it: &mut TSPropertySignature<'a>) {
        walk_ts_property_signature(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_call_signature_declaration(&mut self, it: &mut TSCallSignatureDeclaration<'a>) {
        walk_ts_call_signature_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_construct_signature_declaration(
        &mut self,
        it: &mut TSConstructSignatureDeclaration<'a>,
    ) {
        walk_ts_construct_signature_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_method_signature(&mut self, it: &mut TSMethodSignature<'a>) {
        walk_ts_method_signature(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_operator(&mut self, it: &mut TSTypeOperator<'a>) {
        walk_ts_type_operator(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_predicate(&mut self, it: &mut TSTypePredicate<'a>) {
        walk_ts_type_predicate(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_query(&mut self, it: &mut TSTypeQuery<'a>) {
        walk_ts_type_query(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_reference(&mut self, it: &mut TSTypeReference<'a>) {
        walk_ts_type_reference(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_union_type(&mut self, it: &mut TSUnionType<'a>) {
        walk_ts_union_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_parenthesized_type(&mut self, it: &mut TSParenthesizedType<'a>) {
        walk_ts_parenthesized_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_js_doc_nullable_type(&mut self, it: &mut JSDocNullableType<'a>) {
        walk_js_doc_nullable_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_js_doc_non_nullable_type(&mut self, it: &mut JSDocNonNullableType<'a>) {
        walk_js_doc_non_nullable_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_js_doc_unknown_type(&mut self, it: &mut JSDocUnknownType) {
        walk_js_doc_unknown_type(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_function_body(&mut self, it: &mut FunctionBody<'a>) {
        walk_function_body(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_expression(&mut self, it: &mut AssignmentExpression<'a>) {
        walk_assignment_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_as_expression(&mut self, it: &mut TSAsExpression<'a>) {
        walk_ts_as_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_satisfies_expression(&mut self, it: &mut TSSatisfiesExpression<'a>) {
        walk_ts_satisfies_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_non_null_expression(&mut self, it: &mut TSNonNullExpression<'a>) {
        walk_ts_non_null_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_assertion(&mut self, it: &mut TSTypeAssertion<'a>) {
        walk_ts_type_assertion(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_instantiation_expression(&mut self, it: &mut TSInstantiationExpression<'a>) {
        walk_ts_instantiation_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_computed_member_expression(&mut self, it: &mut ComputedMemberExpression<'a>) {
        walk_computed_member_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_static_member_expression(&mut self, it: &mut StaticMemberExpression<'a>) {
        walk_static_member_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_private_field_expression(&mut self, it: &mut PrivateFieldExpression<'a>) {
        walk_private_field_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_array_assignment_target(&mut self, it: &mut ArrayAssignmentTarget<'a>) {
        walk_array_assignment_target(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_target_with_default(&mut self, it: &mut AssignmentTargetWithDefault<'a>) {
        walk_assignment_target_with_default(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_target_rest(&mut self, it: &mut AssignmentTargetRest<'a>) {
        walk_assignment_target_rest(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_object_assignment_target(&mut self, it: &mut ObjectAssignmentTarget<'a>) {
        walk_object_assignment_target(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_target_property_identifier(
        &mut self,
        it: &mut AssignmentTargetPropertyIdentifier<'a>,
    ) {
        walk_assignment_target_property_identifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_assignment_target_property_property(
        &mut self,
        it: &mut AssignmentTargetPropertyProperty<'a>,
    ) {
        walk_assignment_target_property_property(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_await_expression(&mut self, it: &mut AwaitExpression<'a>) {
        walk_await_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_binary_expression(&mut self, it: &mut BinaryExpression<'a>) {
        walk_binary_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_call_expression(&mut self, it: &mut CallExpression<'a>) {
        walk_call_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_chain_expression(&mut self, it: &mut ChainExpression<'a>) {
        walk_chain_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_class(&mut self, it: &mut Class<'a>) {
        walk_class(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_class_implements(&mut self, it: &mut TSClassImplements<'a>) {
        walk_ts_class_implements(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_class_body(&mut self, it: &mut ClassBody<'a>) {
        walk_class_body(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_static_block(&mut self, it: &mut StaticBlock<'a>) {
        walk_static_block(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_method_definition(&mut self, it: &mut MethodDefinition<'a>) {
        walk_method_definition(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_function(&mut self, it: &mut Function<'a>, flags: ScopeFlags) {
        walk_function(self, it, flags);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_property_definition(&mut self, it: &mut PropertyDefinition<'a>) {
        walk_property_definition(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_accessor_property(&mut self, it: &mut AccessorProperty<'a>) {
        walk_accessor_property(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_conditional_expression(&mut self, it: &mut ConditionalExpression<'a>) {
        walk_conditional_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_expression(&mut self, it: &mut ImportExpression<'a>) {
        walk_import_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_logical_expression(&mut self, it: &mut LogicalExpression<'a>) {
        walk_logical_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_new_expression(&mut self, it: &mut NewExpression<'a>) {
        walk_new_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_object_expression(&mut self, it: &mut ObjectExpression<'a>) {
        walk_object_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_object_property(&mut self, it: &mut ObjectProperty<'a>) {
        walk_object_property(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_parenthesized_expression(&mut self, it: &mut ParenthesizedExpression<'a>) {
        walk_parenthesized_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_sequence_expression(&mut self, it: &mut SequenceExpression<'a>) {
        walk_sequence_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_tagged_template_expression(&mut self, it: &mut TaggedTemplateExpression<'a>) {
        walk_tagged_template_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_this_expression(&mut self, it: &mut ThisExpression) {
        walk_this_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_update_expression(&mut self, it: &mut UpdateExpression<'a>) {
        walk_update_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_yield_expression(&mut self, it: &mut YieldExpression<'a>) {
        walk_yield_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_private_in_expression(&mut self, it: &mut PrivateInExpression<'a>) {
        walk_private_in_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_element(&mut self, it: &mut JSXElement<'a>) {
        walk_jsx_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_opening_element(&mut self, it: &mut JSXOpeningElement<'a>) {
        walk_jsx_opening_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_identifier(&mut self, it: &mut JSXIdentifier<'a>) {
        walk_jsx_identifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_namespaced_name(&mut self, it: &mut JSXNamespacedName<'a>) {
        walk_jsx_namespaced_name(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_member_expression(&mut self, it: &mut JSXMemberExpression<'a>) {
        walk_jsx_member_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_attribute(&mut self, it: &mut JSXAttribute<'a>) {
        walk_jsx_attribute(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_expression_container(&mut self, it: &mut JSXExpressionContainer<'a>) {
        walk_jsx_expression_container(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_empty_expression(&mut self, it: &mut JSXEmptyExpression) {
        walk_jsx_empty_expression(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_fragment(&mut self, it: &mut JSXFragment<'a>) {
        walk_jsx_fragment(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_text(&mut self, it: &mut JSXText<'a>) {
        walk_jsx_text(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_spread_child(&mut self, it: &mut JSXSpreadChild<'a>) {
        walk_jsx_spread_child(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_spread_attribute(&mut self, it: &mut JSXSpreadAttribute<'a>) {
        walk_jsx_spread_attribute(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_jsx_closing_element(&mut self, it: &mut JSXClosingElement<'a>) {
        walk_jsx_closing_element(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_empty_statement(&mut self, it: &mut EmptyStatement) {
        walk_empty_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_expression_statement(&mut self, it: &mut ExpressionStatement<'a>) {
        walk_expression_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_for_in_statement(&mut self, it: &mut ForInStatement<'a>) {
        walk_for_in_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_variable_declaration(&mut self, it: &mut VariableDeclaration<'a>) {
        walk_variable_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_variable_declarator(&mut self, it: &mut VariableDeclarator<'a>) {
        walk_variable_declarator(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_for_of_statement(&mut self, it: &mut ForOfStatement<'a>) {
        walk_for_of_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_for_statement(&mut self, it: &mut ForStatement<'a>) {
        walk_for_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_if_statement(&mut self, it: &mut IfStatement<'a>) {
        walk_if_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_labeled_statement(&mut self, it: &mut LabeledStatement<'a>) {
        walk_labeled_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_return_statement(&mut self, it: &mut ReturnStatement<'a>) {
        walk_return_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_switch_statement(&mut self, it: &mut SwitchStatement<'a>) {
        walk_switch_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_switch_case(&mut self, it: &mut SwitchCase<'a>) {
        walk_switch_case(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_throw_statement(&mut self, it: &mut ThrowStatement<'a>) {
        walk_throw_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_try_statement(&mut self, it: &mut TryStatement<'a>) {
        walk_try_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_catch_clause(&mut self, it: &mut CatchClause<'a>) {
        walk_catch_clause(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_catch_parameter(&mut self, it: &mut CatchParameter<'a>) {
        walk_catch_parameter(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_while_statement(&mut self, it: &mut WhileStatement<'a>) {
        walk_while_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_with_statement(&mut self, it: &mut WithStatement<'a>) {
        walk_with_statement(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_type_alias_declaration(&mut self, it: &mut TSTypeAliasDeclaration<'a>) {
        walk_ts_type_alias_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_interface_declaration(&mut self, it: &mut TSInterfaceDeclaration<'a>) {
        walk_ts_interface_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_interface_heritage(&mut self, it: &mut TSInterfaceHeritage<'a>) {
        walk_ts_interface_heritage(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_interface_body(&mut self, it: &mut TSInterfaceBody<'a>) {
        walk_ts_interface_body(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_enum_declaration(&mut self, it: &mut TSEnumDeclaration<'a>) {
        walk_ts_enum_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_enum_member(&mut self, it: &mut TSEnumMember<'a>) {
        walk_ts_enum_member(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_module_declaration(&mut self, it: &mut TSModuleDeclaration<'a>) {
        walk_ts_module_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_module_block(&mut self, it: &mut TSModuleBlock<'a>) {
        walk_ts_module_block(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_import_equals_declaration(&mut self, it: &mut TSImportEqualsDeclaration<'a>) {
        walk_ts_import_equals_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_external_module_reference(&mut self, it: &mut TSExternalModuleReference<'a>) {
        walk_ts_external_module_reference(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_declaration(&mut self, it: &mut ImportDeclaration<'a>) {
        walk_import_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_specifier(&mut self, it: &mut ImportSpecifier<'a>) {
        walk_import_specifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_default_specifier(&mut self, it: &mut ImportDefaultSpecifier<'a>) {
        walk_import_default_specifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_namespace_specifier(&mut self, it: &mut ImportNamespaceSpecifier<'a>) {
        walk_import_namespace_specifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_with_clause(&mut self, it: &mut WithClause<'a>) {
        walk_with_clause(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_import_attribute(&mut self, it: &mut ImportAttribute<'a>) {
        walk_import_attribute(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_export_all_declaration(&mut self, it: &mut ExportAllDeclaration<'a>) {
        walk_export_all_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_export_default_declaration(&mut self, it: &mut ExportDefaultDeclaration<'a>) {
        walk_export_default_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_export_named_declaration(&mut self, it: &mut ExportNamedDeclaration<'a>) {
        walk_export_named_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_export_specifier(&mut self, it: &mut ExportSpecifier<'a>) {
        walk_export_specifier(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_export_assignment(&mut self, it: &mut TSExportAssignment<'a>) {
        walk_ts_export_assignment(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }

    #[inline]
    fn visit_ts_namespace_export_declaration(&mut self, it: &mut TSNamespaceExportDeclaration<'a>) {
        walk_ts_namespace_export_declaration(self, it);
        it.span = Span::new(it.span.start + self.0, it.span.end + self.0);
    }
}
