//! Level-A signature extraction from the AST.
//!
//! Per parameter: name, **source-level** type text, pointer/reference kind, and a
//! coarse in/out direction inferred from that kind. Source (AST) types are exactly
//! what the C side spells and what the lifted-vs-c2rust harness must bridge
//! (e.g. c2rust `*mut buffer_t` vs CROWN-lifted `Option<Box<buffer_t>>`).
//!
//! Direction here is coarse (pointer-kind only). Read/write dataflow ("level B")
//! is deliberately deferred.

use serde::Serialize;
use syntax::ast::HasName;
use syntax::{ast, AstNode};

#[derive(Serialize)]
pub struct Param {
    pub name: String,
    pub ty: String,
    /// mut_ptr | const_ptr | mut_ref | ref | value
    pub ptr_kind: String,
    /// in | inout
    pub dir: String,
}

#[derive(Serialize)]
pub struct Signature {
    pub params: Vec<Param>,
    pub ret: String,
}

pub fn signature_of(fnode: &ast::Fn) -> Signature {
    let mut params = Vec::new();
    if let Some(pl) = fnode.param_list() {
        if let Some(sp) = pl.self_param() {
            let text = sp.syntax().text().to_string();
            let kind = if text.contains("&mut") {
                "mut_ref"
            } else if text.contains('&') {
                "ref"
            } else {
                "value"
            };
            params.push(mk_param("self".to_string(), text, kind));
        }
        for p in pl.params() {
            let name = p.pat().map(|pat| pat_name(&pat)).unwrap_or_default();
            let ty_ast = p.ty();
            let ty = ty_ast.as_ref().map(|t| t.syntax().text().to_string()).unwrap_or_default();
            let kind = classify(ty_ast.as_ref());
            params.push(mk_param(name, ty, kind));
        }
    }
    let ret = fnode
        .ret_type()
        .and_then(|r| r.ty())
        .map(|t| t.syntax().text().to_string())
        .unwrap_or_else(|| "()".to_string());
    Signature { params, ret }
}

/// The bound identifier, stripped of `mut`/`ref` binding modifiers (e.g. the AST
/// pattern `mut self_0` yields `self_0`). Falls back to the raw pattern text.
fn pat_name(pat: &ast::Pat) -> String {
    match pat {
        ast::Pat::IdentPat(ip) => ip
            .name()
            .map(|n| n.text().to_string())
            .unwrap_or_else(|| pat.syntax().text().to_string()),
        _ => pat.syntax().text().to_string(),
    }
}

fn mk_param(name: String, ty: String, ptr_kind: &str) -> Param {
    let dir = match ptr_kind {
        "mut_ptr" | "mut_ref" => "inout",
        _ => "in",
    };
    Param { name, ty, ptr_kind: ptr_kind.to_string(), dir: dir.to_string() }
}

fn classify(ty: Option<&ast::Type>) -> &'static str {
    match ty {
        Some(ast::Type::PtrType(p)) => {
            if p.mut_token().is_some() {
                "mut_ptr"
            } else {
                "const_ptr"
            }
        }
        Some(ast::Type::RefType(r)) => {
            if r.mut_token().is_some() {
                "mut_ref"
            } else {
                "ref"
            }
        }
        _ => "value",
    }
}
