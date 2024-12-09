use bitflags::bitflags;
use oxc_index::IndexVec;
use oxc_span::{CompactStr, Span};
use oxc_syntax::{node::NodeId, scope::ScopeId};
use rusvelte_ast::js_ast::VariableDeclarationKind;

pub use oxc_syntax::symbol::SymbolId;

bitflags! {
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
    pub struct BindingFlags: u8 {
        const None = 0;
        const Read = 1 << 0;
        const Reassigned = 1 << 1;
        const Mutated = 1 << 2;
        const Updated = Self::Reassigned.bits() | Self::Mutated.bits();
    }
}

impl BindingFlags {
    pub const fn read() -> Self {
        Self::Read
    }

    pub const fn reassigned() -> Self {
        Self::Reassigned
    }

    pub const fn mutated() -> Self {
        Self::Mutated
    }

    pub const fn updated() -> Self {
        Self::Updated
    }

    pub const fn is_read_only(&self) -> bool {
        self.contains(Self::Read)
    }

    pub const fn is_updated(&self) -> bool {
        self.intersects(Self::Updated)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    Normal,
    Prop,
    BindableProp,
    RestProp,
    State,
    RawState,
    Derived,
    Each,
    Snippet,
    StoreSub,
    LegacyReactive,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    Var,
    Let,
    Const,
    Function,
    Import,
    Param,
    RestParam,
    Synthetic,
}

impl From<VariableDeclarationKind> for DeclarationKind {
    fn from(value: VariableDeclarationKind) -> Self {
        match value {
            VariableDeclarationKind::Var => Self::Var,
            VariableDeclarationKind::Const => Self::Const,
            VariableDeclarationKind::Let => Self::Let,
            VariableDeclarationKind::Using => todo!(),
            VariableDeclarationKind::AwaitUsing => Self::Import,
        }
    }
}

#[derive(Debug)]
pub struct Binding {
    // VariableDeclarator
    node_id: NodeId,
    #[allow(unused)]
    scope_id: ScopeId,

    kind: BindingKind,
    #[allow(unused)]
    declaration_kind: DeclarationKind,
    #[allow(unused)]
    is_called: bool,
    pub binding_flags: BindingFlags,
}

impl Binding {
    pub fn new(
        node_id: NodeId,
        scope_id: ScopeId,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
    ) -> Self {
        Self {
            node_id,
            scope_id,
            kind,
            declaration_kind,
            is_called: false,
            binding_flags: BindingFlags::None,
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn kind(&self) -> BindingKind {
        self.kind
    }

    pub fn is_init_by_state(&self) -> bool {
        self.kind == BindingKind::State
    }
}

#[derive(Debug, Default)]
pub struct BindingTable {
    names: IndexVec<SymbolId, CompactStr>,
    spans: IndexVec<SymbolId, Span>,
    declarations: IndexVec<SymbolId, Binding>,
}

impl BindingTable {
    pub fn create_symbol<T: Into<CompactStr>>(
        &mut self,
        span: Span,
        name: T,
        scope_id: ScopeId,
        node_id: NodeId,
        kind: BindingKind,
        declaration_kind: DeclarationKind,
    ) -> SymbolId {
        self.spans.push(span);
        self.names.push(name.into());
        self.declarations
            .push(Binding::new(node_id, scope_id, kind, declaration_kind))
    }

    pub fn names(&self) -> &IndexVec<SymbolId, CompactStr> {
        &self.names
    }

    pub fn get_binding_mut(&mut self, symbol_id: SymbolId) -> &mut Binding {
        &mut self.declarations[symbol_id]
    }

    pub fn get_binding(&self, symbol_id: SymbolId) -> &Binding {
        &self.declarations[symbol_id]
    }

    pub fn get_name(&self, symbol_id: SymbolId) -> &str {
        &self.names[symbol_id]
    }
}
