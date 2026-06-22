//! Extract a Rust call graph from a single .rs file using `syn`.
//!
//! Mirrors the C-side extractor (tools/stu_selector/callgraph.py): it emits function
//! definitions, resolved-by-name call edges, and unresolved/indirect call sites. Graph
//! algorithms (SCC, condensation) are done in Python so they are shared with the C side;
//! this tool only does faithful AST extraction.
//!
//! Usage: rust_callgraph <file.rs>   (prints JSON to stdout)

use std::collections::BTreeSet;
use std::fs;

use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, ExprMethodCall, ImplItemFn, ItemFn};

#[derive(Serialize)]
struct FnRec {
    name: String,
    line: usize,
}

#[derive(Serialize)]
struct Edge {
    from: String,
    to: String,
}

#[derive(Serialize)]
struct Indirect {
    from: String,
    line: usize,
    kind: String,
}

#[derive(Serialize)]
struct Output {
    functions: Vec<FnRec>,
    raw_edges: Vec<Edge>,
    indirect_calls: Vec<Indirect>,
}

struct Cg {
    functions: Vec<FnRec>,
    edges: BTreeSet<(String, String)>,
    indirect: Vec<Indirect>,
    current: Vec<String>,
}

impl Cg {
    fn new() -> Self {
        Cg {
            functions: Vec::new(),
            edges: BTreeSet::new(),
            indirect: Vec::new(),
            current: Vec::new(),
        }
    }

    fn enter_fn(&mut self, name: String, line: usize) {
        self.functions.push(FnRec {
            name: name.clone(),
            line,
        });
        self.current.push(name);
    }
}

impl<'ast> Visit<'ast> for Cg {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let line = node.sig.ident.span().start().line;
        self.enter_fn(node.sig.ident.to_string(), line);
        visit::visit_block(self, &node.block);
        self.current.pop();
    }

    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        let line = node.sig.ident.span().start().line;
        self.enter_fn(node.sig.ident.to_string(), line);
        visit::visit_block(self, &node.block);
        self.current.pop();
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Some(cur) = self.current.last().cloned() {
            if let Expr::Path(p) = &*node.func {
                if let Some(seg) = p.path.segments.last() {
                    self.edges.insert((cur, seg.ident.to_string()));
                }
            } else {
                // Calling a non-path expression: function pointer / closure / etc.
                self.indirect.push(Indirect {
                    from: cur,
                    line: node.func.span().start().line,
                    kind: "fn_pointer_or_expr".to_string(),
                });
            }
        }
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        if let Some(cur) = self.current.last().cloned() {
            // Method dispatch cannot be resolved to a free function statically.
            self.indirect.push(Indirect {
                from: cur,
                line: node.method.span().start().line,
                kind: format!("method:{}", node.method),
            });
        }
        visit::visit_expr_method_call(self, node);
    }
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: rust_callgraph <file.rs>");
            std::process::exit(2);
        }
    };
    let src = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    let file = syn::parse_file(&src).unwrap_or_else(|e| {
        eprintln!("parse error in {path}: {e}");
        std::process::exit(1);
    });

    let mut cg = Cg::new();
    cg.visit_file(&file);

    let out = Output {
        functions: cg.functions,
        raw_edges: cg
            .edges
            .into_iter()
            .map(|(from, to)| Edge { from, to })
            .collect(),
        indirect_calls: cg.indirect,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
