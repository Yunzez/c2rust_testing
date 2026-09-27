//! Operator histogram per function — a rename-invariant signal for matching that
//! distinguishes structurally identical functions differing only by an operator
//! (e.g. `a / b` vs `a % b`). Canonical operator symbols are shared with the C side
//! (c_analyzer.py). Compound assignment `OP=` is normalized to the underlying `OP`;
//! `~` and `!` both map to `!` (Rust uses `!` for both logical and bitwise not);
//! deref/neg/addr are skipped (covered by metrics / too noisy cross-language).

use std::collections::BTreeMap;

use syntax::ast::{self, BinaryOp, CmpOp, Ordering, UnaryOp};
use syntax::AstNode;

pub type OpHist = BTreeMap<String, u32>;

pub fn ops_of(fnode: &ast::Fn) -> OpHist {
    let mut h = OpHist::new();
    let body = match fnode.body() {
        Some(b) => b,
        None => return h,
    };
    for node in body.syntax().descendants() {
        if let Some(b) = ast::BinExpr::cast(node.clone()) {
            if let Some(sym) = b.op_kind().and_then(binop_sym) {
                *h.entry(sym).or_insert(0) += 1;
            }
        } else if let Some(p) = ast::PrefixExpr::cast(node.clone()) {
            if p.op_kind() == Some(UnaryOp::Not) {
                *h.entry("!".to_string()).or_insert(0) += 1;
            } else if p.op_kind() == Some(UnaryOp::Neg) {
                // unary negation — re-included (was skipped as "noisy"): it is the exact
                // discriminator of the lil fnc_inc/fnc_dec twin (-amount), preserved on
                // both sides. Validated against the 57-cell regression harness.
                *h.entry("neg".to_string()).or_insert(0) += 1;
            }
        }
    }
    h
}

fn binop_sym(op: BinaryOp) -> Option<String> {
    Some(match op {
        BinaryOp::ArithOp(a) => a.to_string(),
        BinaryOp::LogicOp(l) => l.to_string(),
        BinaryOp::CmpOp(CmpOp::Eq { negated }) => if negated { "!=" } else { "==" }.to_string(),
        BinaryOp::CmpOp(CmpOp::Ord { ordering, strict }) => match (ordering, strict) {
            (Ordering::Less, true) => "<",
            (Ordering::Less, false) => "<=",
            (Ordering::Greater, true) => ">",
            (Ordering::Greater, false) => ">=",
        }
        .to_string(),
        BinaryOp::Assignment { op: Some(a) } => a.to_string(),
        BinaryOp::Assignment { op: None } => return None,
    })
}
