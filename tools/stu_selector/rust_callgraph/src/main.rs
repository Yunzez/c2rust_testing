//! Extract a Rust call graph + per-function metrics from a single .rs file using `syn`.
//!
//! Rust-side mirror of tools/stu_selector/callgraph.py + the C metrics in features.py.
//! Emits, per function: call edges (by name), indirect/method calls, and a metric vector
//! (cyclomatic complexity, statement count, loops, max loop nesting, pointer derefs,
//! allocation calls, method calls). Graph algorithms stay in Python.
//!
//! Usage: rust_callgraph <file.rs>   (prints JSON to stdout)

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;

use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    BinOp, Expr, ExprCall, ExprMethodCall, ImplItemFn, ItemFn, Stmt, UnOp,
};

#[derive(Serialize, Default, Clone)]
struct Metrics {
    cyclomatic: u32,
    stmts: u32,
    loops: u32,
    max_loop_depth: u32,
    derefs: u32,
    allocs: u32,
    method_calls: u32,
}

#[derive(Serialize)]
struct FnRec {
    name: String,
    line: usize,
    metrics: Metrics,
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

const ALLOC_FNS: &[&str] = &["malloc", "calloc", "realloc", "free", "alloc", "dealloc"];

struct Cg {
    order: Vec<(String, usize)>,
    metrics: HashMap<String, Metrics>,
    edges: BTreeSet<(String, String)>,
    indirect: Vec<Indirect>,
    current: Vec<String>,
    loop_depth: u32,
}

impl Cg {
    fn new() -> Self {
        Cg {
            order: Vec::new(),
            metrics: HashMap::new(),
            edges: BTreeSet::new(),
            indirect: Vec::new(),
            current: Vec::new(),
            loop_depth: 0,
        }
    }

    fn cur(&self) -> Option<String> {
        self.current.last().cloned()
    }

    fn m(&mut self) -> Option<&mut Metrics> {
        let c = self.current.last()?.clone();
        self.metrics.get_mut(&c)
    }

    fn enter_fn(&mut self, name: String, line: usize) {
        // cyclomatic complexity starts at 1.
        self.metrics.entry(name.clone()).or_insert(Metrics {
            cyclomatic: 1,
            ..Default::default()
        });
        self.order.push((name.clone(), line));
        self.current.push(name);
    }

    fn visit_loop_body<F: FnOnce(&mut Self)>(&mut self, f: F) {
        self.loop_depth += 1;
        let depth = self.loop_depth;
        if let Some(m) = self.m() {
            m.loops += 1;
            m.cyclomatic += 1;
            if depth > m.max_loop_depth {
                m.max_loop_depth = depth;
            }
        }
        f(self);
        self.loop_depth -= 1;
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

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        if let Some(m) = self.m() {
            m.stmts += 1;
        }
        visit::visit_stmt(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        if let Some(m) = self.m() {
            m.cyclomatic += 1;
        }
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        if let Some(m) = self.m() {
            // each arm beyond the first is an extra path
            m.cyclomatic += node.arms.len().saturating_sub(1) as u32;
        }
        visit::visit_expr_match(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.visit_loop_body(|s| visit::visit_expr_while(s, node));
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.visit_loop_body(|s| visit::visit_expr_for_loop(s, node));
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.visit_loop_body(|s| visit::visit_expr_loop(s, node));
    }

    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        if matches!(node.op, BinOp::And(_) | BinOp::Or(_)) {
            if let Some(m) = self.m() {
                m.cyclomatic += 1;
            }
        }
        visit::visit_expr_binary(self, node);
    }

    fn visit_expr_unary(&mut self, node: &'ast syn::ExprUnary) {
        if matches!(node.op, UnOp::Deref(_)) {
            if let Some(m) = self.m() {
                m.derefs += 1;
            }
        }
        visit::visit_expr_unary(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast syn::ExprTry) {
        if let Some(m) = self.m() {
            m.cyclomatic += 1;
        }
        visit::visit_expr_try(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Some(cur) = self.cur() {
            if let Expr::Path(p) = &*node.func {
                if let Some(seg) = p.path.segments.last() {
                    let callee = seg.ident.to_string();
                    if ALLOC_FNS.contains(&callee.as_str()) {
                        if let Some(m) = self.m() {
                            m.allocs += 1;
                        }
                    }
                    self.edges.insert((cur, callee));
                }
            } else {
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
        if let Some(cur) = self.cur() {
            if let Some(m) = self.m() {
                m.method_calls += 1;
            }
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

    let functions = cg
        .order
        .iter()
        .map(|(name, line)| FnRec {
            name: name.clone(),
            line: *line,
            metrics: cg.metrics.get(name).cloned().unwrap_or_default(),
        })
        .collect();

    let out = Output {
        functions,
        raw_edges: cg
            .edges
            .into_iter()
            .map(|(from, to)| Edge { from, to })
            .collect(),
        indirect_calls: cg.indirect,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
