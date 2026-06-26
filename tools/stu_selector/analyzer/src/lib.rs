//! rust-analyzer-powered crate analyzer for the STU selector.
//!
//! Loads a whole Cargo crate the way rust-analyzer does (all modules, name/type
//! resolution) and extracts a call graph. This is the Rust-side counterpart to the
//! C-side libclang + compile_commands analysis in `callgraph.py` — symmetric: a real
//! frontend over the whole crate, not a single-file syntactic parse.
//!
//! Phase 1: functions (name + line), resolved call edges, and unresolved/indirect
//! calls. Signature and structural I/O fingerprints come in later phases.

mod io;
mod metrics;
mod ops;
mod signature;

use std::collections::HashSet;
use std::path::Path;

use hir::{CallableKind, Crate, Module, Semantics};
use ide::{AnalysisHost, RootDatabase};
use ide_db::base_db::SourceDatabase;
use ide_db::EditionedFileId;
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use project_model::{CargoConfig, RustLibSource};
use serde::Serialize;
use syntax::ast::HasName;
use syntax::{ast, AstNode};

#[derive(Serialize)]
pub struct FnRec {
    pub name: String,
    pub line: usize,
    pub signature: signature::Signature,
    pub io: io::Io,
    pub ops: ops::OpHist,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<metrics::Metrics>,
}

#[derive(Serialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct Indirect {
    pub from: String,
    pub line: usize,
    pub kind: String,
}

#[derive(Serialize)]
pub struct Output {
    pub functions: Vec<FnRec>,
    pub raw_edges: Vec<Edge>,
    pub indirect_calls: Vec<Indirect>,
}

/// A loaded crate, ready to analyze. Owns the rust-analyzer database.
pub struct AnalyzedCrate {
    host: AnalysisHost,
}

/// Load a Cargo crate the way rust-analyzer does. A load failure means the
/// translator produced a crate that does not even load — that is a finding about
/// the translator, surfaced as an `Err` (the CLI maps it to a structured error).
pub fn load_crate(dir: &Path) -> anyhow::Result<AnalyzedCrate> {
    let cargo_config = CargoConfig {
        sysroot: Some(RustLibSource::Discover),
        all_targets: true,
        set_test: true,
        ..Default::default()
    };
    let load_cargo_config = LoadCargoConfig {
        load_out_dirs_from_check: true,
        // c2rust / CROWN output has no proc-macros; skip the sysroot proc-macro
        // server to avoid a host rustc<->ra_ap ABI mismatch (ApiVersionCheck).
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
        num_worker_threads: 1,
        proc_macro_processes: 1,
    };
    let (db, _vfs, _proc_macro) =
        load_workspace_at(dir, &cargo_config, &load_cargo_config, &|_| {})?;
    Ok(AnalyzedCrate { host: AnalysisHost::with_database(db) })
}

impl AnalyzedCrate {
    pub fn analyze(&self, enable_metrics: bool) -> Output {
        let db = self.host.raw_database();
        // The next-gen trait solver (used by `type_of_expr`/`as_callable`) requires
        // the salsa db to be attached to the current thread.
        hir::attach_db(db, || self.analyze_inner(db, enable_metrics))
    }

    fn analyze_inner(&self, db: &RootDatabase, enable_metrics: bool) -> Output {
        let sema = Semantics::new(db);

        let mut out =
            Output { functions: Vec::new(), raw_edges: Vec::new(), indirect_calls: Vec::new() };
        let mut seen: HashSet<String> = HashSet::new();

        for efile in local_files(db) {
            // Parse THROUGH Semantics so call expressions can be type-resolved
            // (`type_of_expr` only accepts nodes derived from this Semantics).
            let source_file = sema.parse(efile);
            let vfile = efile.file_id(db);
            let li = ide_db::line_index(db, vfile);

            // Every `ast::Fn` in the file: free fns AND `impl` assoc fns / nested fns.
            for node in source_file.syntax().descendants() {
                let fnode = match ast::Fn::cast(node) {
                    Some(f) => f,
                    None => continue,
                };
                let func: hir::Function = match sema.to_def(&fnode) {
                    Some(f) => f,
                    None => continue,
                };
                let name = func.name(db).as_str().to_owned();

                // Only list actual definitions (with a body); skip `extern "C"`
                // declarations of external symbols (libc printf/strcmp/...). The C
                // side likewise lists definitions only.
                let body = match fnode.body() {
                    Some(b) => b,
                    None => continue,
                };

                let name_offset = fnode
                    .name()
                    .map(|n| n.syntax().text_range().start())
                    .unwrap_or_else(|| fnode.syntax().text_range().start());
                let line = li.line_col(name_offset).line as usize + 1;
                if seen.insert(name.clone()) {
                    out.functions.push(FnRec {
                        name: name.clone(),
                        line,
                        signature: signature::signature_of(&fnode),
                        io: io::io_of(db, func),
                        ops: ops::ops_of(&fnode),
                        metrics: if enable_metrics {
                            Some(metrics::metrics_of(&fnode))
                        } else {
                            None
                        },
                    });
                }
                for n in body.syntax().descendants() {
                    if let Some(call) = ast::CallExpr::cast(n.clone()) {
                        let resolved = call
                            .expr()
                            .as_ref()
                            .and_then(|e| sema.type_of_expr(e))
                            .and_then(|t| t.original.as_callable(db))
                            .map(|c| c.kind());
                        match resolved {
                            Some(CallableKind::Function(callee)) => out.raw_edges.push(Edge {
                                from: name.clone(),
                                to: callee.name(db).as_str().to_owned(),
                            }),
                            _ => out.indirect_calls.push(Indirect {
                                from: name.clone(),
                                line: li.line_col(n.text_range().start()).line as usize + 1,
                                kind: "call_unresolved".to_owned(),
                            }),
                        }
                    } else if let Some(mc) = ast::MethodCallExpr::cast(n.clone()) {
                        match sema.resolve_method_call(&mc) {
                            Some(callee) => out.raw_edges.push(Edge {
                                from: name.clone(),
                                to: callee.name(db).as_str().to_owned(),
                            }),
                            None => out.indirect_calls.push(Indirect {
                                from: name.clone(),
                                line: li.line_col(n.text_range().start()).line as usize + 1,
                                kind: "method_unresolved".to_owned(),
                            }),
                        }
                    }
                }
            }
        }
        out
    }
}

/// Distinct local (non-library) source files, as `EditionedFileId`s suitable for
/// `Semantics::parse`. Collected from every module of every local crate.
fn local_files(db: &RootDatabase) -> Vec<EditionedFileId> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for m in local_modules(db) {
        let efile = m.definition_source_file_id(db).original_file(db);
        if seen.insert(efile.file_id(db)) {
            out.push(efile);
        }
    }
    out
}

/// All modules belonging to local (non-library) crates, walked from each local
/// crate's root module. Mirrors rust-analyzer's own CLI traversal.
fn local_modules(db: &RootDatabase) -> Vec<Module> {
    let mut worklist: Vec<Module> = Crate::all(db)
        .into_iter()
        .filter(|krate| !crate_is_library(db, *krate))
        .map(|krate| krate.root_module(db))
        .collect();
    let mut out = Vec::new();
    while let Some(m) = worklist.pop() {
        out.push(m);
        worklist.extend(m.children(db));
    }
    out
}

fn crate_is_library(db: &RootDatabase, krate: Crate) -> bool {
    let file_id = krate.root_module(db).definition_source_file_id(db).original_file(db);
    let source_root = db.file_source_root(file_id.file_id(db)).source_root_id(db);
    db.source_root(source_root).source_root(db).is_library
}
