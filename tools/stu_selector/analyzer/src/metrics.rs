//! Structural per-function metrics, computed from the AST. Off by default, emitted
//! only under `--enable-metrics`. Same 8 fields as the legacy syn tool; intended
//! (with `io`) as a future name-independent matching signal, not exact parity.

use serde::Serialize;
use syntax::{ast, AstNode, SyntaxKind, SyntaxNode};

const ALLOC_FNS: &[&str] = &["malloc", "calloc", "realloc", "free", "alloc", "dealloc"];

#[derive(Serialize, Default)]
pub struct Metrics {
    pub cyclomatic: u32,
    pub stmts: u32,
    pub nodes: u32,
    pub loops: u32,
    pub max_loop_depth: u32,
    pub derefs: u32,
    pub allocs: u32,
    pub method_calls: u32,
}

pub fn metrics_of(fnode: &ast::Fn) -> Metrics {
    let mut m = Metrics { cyclomatic: 1, ..Default::default() };
    let body = match fnode.body() {
        Some(b) => b,
        None => return m,
    };
    for node in body.syntax().descendants() {
        match node.kind() {
            SyntaxKind::LET_STMT | SyntaxKind::EXPR_STMT => m.stmts += 1,
            SyntaxKind::FOR_EXPR | SyntaxKind::WHILE_EXPR | SyntaxKind::LOOP_EXPR => {
                m.loops += 1;
                m.cyclomatic += 1;
                let d = loop_depth(&node);
                if d > m.max_loop_depth {
                    m.max_loop_depth = d;
                }
            }
            SyntaxKind::IF_EXPR | SyntaxKind::MATCH_ARM => m.cyclomatic += 1,
            SyntaxKind::METHOD_CALL_EXPR => m.method_calls += 1,
            SyntaxKind::PREFIX_EXPR => {
                if let Some(pe) = ast::PrefixExpr::cast(node.clone()) {
                    if pe.op_kind() == Some(ast::UnaryOp::Deref) {
                        m.derefs += 1;
                    }
                }
            }
            SyntaxKind::CALL_EXPR => {
                if is_alloc_call(&node) {
                    m.allocs += 1;
                }
            }
            _ => {}
        }
        if ast::Expr::can_cast(node.kind()) || ast::Stmt::can_cast(node.kind()) {
            m.nodes += 1;
        }
    }
    m
}

fn loop_depth(node: &SyntaxNode) -> u32 {
    let mut depth = 1;
    let mut parent = node.parent();
    while let Some(n) = parent {
        if matches!(n.kind(), SyntaxKind::FOR_EXPR | SyntaxKind::WHILE_EXPR | SyntaxKind::LOOP_EXPR) {
            depth += 1;
        }
        parent = n.parent();
    }
    depth
}

fn is_alloc_call(node: &SyntaxNode) -> bool {
    let call = match ast::CallExpr::cast(node.clone()) {
        Some(c) => c,
        None => return false,
    };
    let callee = match call.expr() {
        Some(e) => e.syntax().text().to_string(),
        None => return false,
    };
    let last = callee.rsplit("::").next().unwrap_or(&callee).trim();
    ALLOC_FNS.contains(&last)
}
