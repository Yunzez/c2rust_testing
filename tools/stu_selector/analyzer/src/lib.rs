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
mod consts;
mod metrics;
mod ops;
mod signature;

use std::collections::HashSet;
use std::path::Path;

use hir::{AsAssocItem, CallableKind, Crate, Module, Semantics};

/// Stable node identity for a function, used for dedup + call-graph edges + truth keying.
///
/// FIX (hir-id backlog): the analyzer previously keyed functions by their bare name, so two
/// functions with the same name (e.g. `QuadPoint::new`, `QuadNode::new`, `QuadTree::new` — every
/// struct's constructor) collided in the `seen: HashSet<String>` and all but the first were
/// SILENTLY DROPPED. That caps recall on any idiomatic-Rust output (LLM transpilers) with multiple
/// same-named methods. We disambiguate an associated method as `Self::method` (the receiver type
/// comes from hir resolution); free functions keep their bare name so name-equality against the C
/// side (whose functions are all free) is preserved for name-preserving translators.
fn fn_id(db: &RootDatabase, func: hir::Function) -> String {
    let name = func.name(db).as_str().to_owned();
    if let Some(assoc) = func.as_assoc_item(db) {
        if let hir::AssocItemContainer::Impl(imp) = assoc.container(db) {
            if let Some(adt) = imp.self_ty(db).as_adt() {
                return format!("{}::{}", adt.name(db).as_str(), name);
            }
        }
    }
    name
}
use ide::{AnalysisHost, RootDatabase};
use ide_db::base_db::SourceDatabase;
use ide_db::EditionedFileId;
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use project_model::{CargoConfig, RustLibSource};
use serde::Serialize;
use syntax::ast::{HasAttrs, HasName};
use syntax::{ast, AstNode};

#[derive(Serialize)]
pub struct FnRec {
    pub name: String,
    pub line: usize,
    pub signature: signature::Signature,
    pub io: io::Io,
    pub ops: ops::OpHist,
    pub consts: Vec<String>,
    pub strings: Vec<String>,
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
pub struct Excluded {
    pub name: String,
    /// Why it was dropped: `"test"` or `"trait_boilerplate:<Trait>"`.
    pub reason: String,
}

#[derive(Serialize)]
pub struct Output {
    pub functions: Vec<FnRec>,
    pub raw_edges: Vec<Edge>,
    pub indirect_calls: Vec<Indirect>,
    /// Rust-only nodes excluded from the candidate set AND the topology graph because
    /// they are not a C-function translation target: test scaffolding (`#[test]` /
    /// `#[cfg(test)]`) and boilerplate trait impls (`impl Default/Clone/Debug/...`).
    /// Empirically these have no C counterpart and POISON similarity propagation
    /// (bignum: topology gives 0 gain with test nodes in). Reported, not silently dropped.
    pub excluded_scaffolding: Vec<Excluded>,
}

/// True if this fn is test/bench scaffolding: a `#[test]`/`#[bench]` attribute on the
/// fn itself, or membership in any `#[cfg(test)]` module ancestor. Such code is never
/// part of the translation surface.
fn is_test_scaffolding(fnode: &ast::Fn) -> bool {
    fn marks_test(attrs: impl Iterator<Item = ast::Attr>) -> bool {
        for a in attrs {
            let t: String =
                a.syntax().text().to_string().chars().filter(|c| !c.is_whitespace()).collect();
            if t == "#[test]" || t == "#[bench]" || t.contains("cfg(test)") {
                return true;
            }
        }
        false
    }
    if marks_test(fnode.attrs()) {
        return true;
    }
    fnode
        .syntax()
        .ancestors()
        .filter_map(ast::Module::cast)
        .any(|m| marks_test(m.attrs()))
}

/// Std/derive boilerplate traits whose impl methods are Rust idiom, never the
/// translation of a C function. Deliberately EXCLUDES arithmetic/operator traits
/// (Add/Sub/Mul/Index/Iterator/...) and Display-of-real-logic stays a judgment call —
/// these listed traits are the safe, unambiguous boilerplate set.
const BOILERPLATE_TRAITS: &[&str] = &[
    "Default", "Clone", "Copy", "Debug", "Display", "Hash", "PartialEq", "Eq", "PartialOrd",
    "Ord", "From", "Into", "TryFrom", "TryInto", "Serialize", "Deserialize", "Drop",
];

/// If this fn is a method of a boilerplate trait impl (`impl Default for T { fn default }`,
/// `impl Debug for T { fn fmt }`, ...), return the trait's name. Inherent `impl T` methods
/// and non-boilerplate trait impls (e.g. `impl Add`) return None — they may be real targets.
fn boilerplate_trait(fnode: &ast::Fn) -> Option<String> {
    // A fn's nearest ancestor `impl` is the one it belongs to (impls don't nest).
    let imp = fnode.syntax().ancestors().find_map(ast::Impl::cast)?;
    let trait_ty = imp.trait_()?; // Some only for `impl Trait for Type`
    let name = match trait_ty {
        ast::Type::PathType(p) => p.path()?.segment()?.name_ref()?.text().to_string(),
        _ => return None,
    };
    BOILERPLATE_TRAITS.contains(&name.as_str()).then_some(name)
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

        let mut out = Output {
            functions: Vec::new(),
            raw_edges: Vec::new(),
            indirect_calls: Vec::new(),
            excluded_scaffolding: Vec::new(),
        };
        let mut seen: HashSet<String> = HashSet::new();
        let mut excluded_seen: HashSet<String> = HashSet::new();

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

                // Rust-only non-targets (test scaffolding, boilerplate trait impls,
                // locally-nested helper fns): keep out of BOTH the candidate set and the
                // topology graph (skip the body walk -> no edges), report with a reason.
                //
                // A fn whose ancestors include another `ast::Fn` is defined inside that
                // fn's body. C has no nested functions, so such locals have no C
                // counterpart and only act as distractors during matching (and can collide
                // by name with a top-level fn). Their call edges are NOT lost: the
                // enclosing fn's recursive body walk already attributes them to the parent.
                let is_nested_local =
                    fnode.syntax().ancestors().skip(1).any(|a| ast::Fn::cast(a).is_some());
                let exclude_reason = if is_test_scaffolding(&fnode) {
                    Some("test".to_owned())
                } else if is_nested_local {
                    Some("nested_local".to_owned())
                } else {
                    boilerplate_trait(&fnode).map(|t| format!("trait_boilerplate:{t}"))
                };
                if let Some(reason) = exclude_reason {
                    if excluded_seen.insert(name.clone()) {
                        out.excluded_scaffolding.push(Excluded { name, reason });
                    }
                    continue;
                }

                let name_offset = fnode
                    .name()
                    .map(|n| n.syntax().text_range().start())
                    .unwrap_or_else(|| fnode.syntax().text_range().start());
                let line = li.line_col(name_offset).line as usize + 1;
                // Node identity: `Self::method` for impl methods, bare name for free fns.
                let id = fn_id(db, func);
                if seen.insert(id.clone()) {
                    out.functions.push(FnRec {
                        name: id.clone(),
                        line,
                        signature: signature::signature_of(&fnode),
                        io: io::io_of(db, func),
                        ops: ops::ops_of(&fnode),
                        consts: consts::consts_of(&fnode),
                        strings: consts::strings_of(&fnode),
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
                                from: id.clone(),
                                to: fn_id(db, callee),
                            }),
                            _ => out.indirect_calls.push(Indirect {
                                from: id.clone(),
                                line: li.line_col(n.text_range().start()).line as usize + 1,
                                kind: "call_unresolved".to_owned(),
                            }),
                        }
                    } else if let Some(mc) = ast::MethodCallExpr::cast(n.clone()) {
                        match sema.resolve_method_call(&mc) {
                            Some(callee) => out.raw_edges.push(Edge {
                                from: id.clone(),
                                to: fn_id(db, callee),
                            }),
                            None => out.indirect_calls.push(Indirect {
                                from: id.clone(),
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
