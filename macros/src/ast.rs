/*
    Appellation: ast <module>
    Created At: 2025.12.18:07:44:04
    Contrib: @FL03
*/
mod impl_ast_wrapper_ops;

use syn::token::Impl;
use syn::{AngleBracketedGenericArguments, Ident, WhereClause};

/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `impl A { Add.add, Sub.sub }` or `impl B.field { Add.add, Sub.sub }`
pub struct WrapperOpsAst {
    pub _impl: Impl,
    pub _generics: Option<AngleBracketedGenericArguments>,
    pub target: Ident,
    pub field: Option<Ident>,
    pub _where: Option<WhereClause>,
    pub ops: Vec<(Ident, Ident)>,
}
