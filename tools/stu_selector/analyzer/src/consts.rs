//! signal-C: the set of TYPE-TAG / enum-variant tokens a function constructs — the
//! name-independent discriminator for flat leaf constructors that share io-shape and have
//! no call topology (e.g. an LLM's `JsonValue::Null` / `Bool(true)` / `Number{..}` family,
//! which maps to C's `cJSON_Create*` `#define` tags). Mirrors c_analyzer.py `consts_of`:
//! we collect PascalCase path last-segments (enum variants / constructors) and bool
//! literals, normalize each to its last `::`/`_` segment lowercased, and jaccard the sets
//! cross-language. So Rust `JsonValue::Number` and C macro `cJSON_Number` both -> `number`.

use std::collections::BTreeSet;

use syntax::ast::{self};
use syntax::AstNode;

/// Common std variants/types that carry no domain signal — dropped to limit cross-language
/// false overlap (their normalized forms).
const STOP: &[&str] = &[
    "self", "super", "crate", "some", "none", "ok", "err", "result", "option", "vec",
    "box", "default", "new", "from", "into", "clone",
];

fn norm_tag(s: &str) -> String {
    // last `_`/`::` segment, lowercased (mirror _norm_tag in c_analyzer.py)
    s.rsplit("::").next().unwrap_or(s).rsplit('_').next().unwrap_or(s).to_lowercase()
}

fn is_pascal(name: &str) -> bool {
    name.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}

fn push_path(set: &mut BTreeSet<String>, path: Option<ast::Path>) {
    if let Some(p) = path {
        if let Some(seg) = p.segment() {
            if let Some(nr) = seg.name_ref() {
                let raw = nr.text().to_string();
                if is_pascal(&raw) {
                    let t = norm_tag(&raw);
                    if !STOP.contains(&t.as_str()) {
                        set.insert(t);
                    }
                }
            }
        }
    }
}

/// signal-S: string literals referenced in the body (BinDiff-style string refs). Source
/// text with quotes/raw-markers stripped, escapes kept, so C `"\n"` == Rust `"\n"`.
/// Mirrors c_analyzer.py strings_of.
pub fn strings_of(fnode: &ast::Fn) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    let body = match fnode.body() {
        Some(b) => b,
        None => return Vec::new(),
    };
    for node in body.syntax().descendants() {
        if let Some(lit) = ast::Literal::cast(node.clone()) {
            if let ast::LiteralKind::String(_) = lit.kind() {
                let t = lit.syntax().text().to_string();
                let t = t
                    .trim_start_matches('r')
                    .trim_matches('#')
                    .trim_matches('"')
                    .to_string();
                if t.len() >= 2 {
                    set.insert(t);
                }
            }
        }
    }
    set.into_iter().collect()
}

pub fn consts_of(fnode: &ast::Fn) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    let body = match fnode.body() {
        Some(b) => b,
        None => return Vec::new(),
    };
    for node in body.syntax().descendants() {
        if let Some(pe) = ast::PathExpr::cast(node.clone()) {
            push_path(&mut set, pe.path());
        } else if let Some(re) = ast::RecordExpr::cast(node.clone()) {
            push_path(&mut set, re.path());
        } else if let Some(lit) = ast::Literal::cast(node.clone()) {
            if let ast::LiteralKind::Bool(b) = lit.kind() {
                set.insert(if b { "true" } else { "false" }.to_string());
            }
        }
    }
    set.into_iter().collect()
}
