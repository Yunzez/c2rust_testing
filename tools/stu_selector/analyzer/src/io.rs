//! Structural input/output fingerprints (hir-resolved), for future
//! name-independent matching (e.g. after an LLM transpiler renames things).
//!
//! Each input param and the return type get a resolved display `ty` (nominal) and
//! a structural `shape`: pointers/refs become `*`/`&`, generic containers
//! (Option/Box/Vec) keep their std name with decomposed arguments, and user structs
//! are decomposed into their field shapes — so the primitive content survives
//! renaming. Depth-capped with a visited set so self-referential types terminate.

use std::collections::HashSet;

use hir::db::HirDatabase;
use hir::{Adt, DisplayTarget, Field, HasCrate, HirDisplay, Type};
use serde::Serialize;

const MAX_DEPTH: usize = 6;

#[derive(Serialize)]
pub struct IoType {
    pub ty: String,
    pub shape: String,
}

#[derive(Serialize)]
pub struct Io {
    pub inputs: Vec<IoType>,
    pub output: IoType,
}

pub fn io_of(db: &dyn HirDatabase, f: hir::Function) -> Io {
    let target = DisplayTarget::from_crate(db, f.krate(db).into());
    let inputs = f
        .params_without_self(db)
        .iter()
        .map(|p| io_type(db, target, p.ty()))
        .collect();
    let output = io_type(db, target, &f.ret_type(db));
    Io { inputs, output }
}

fn io_type(db: &dyn HirDatabase, target: DisplayTarget, ty: &Type) -> IoType {
    IoType {
        ty: ty.display(db, target).to_string(),
        shape: shape(db, target, ty, 0, &mut HashSet::new()),
    }
}

fn shape(
    db: &dyn HirDatabase,
    target: DisplayTarget,
    ty: &Type,
    depth: usize,
    seen: &mut HashSet<String>,
) -> String {
    if depth >= MAX_DEPTH {
        return "_".to_string();
    }
    if let Some((inner, _)) = ty.as_raw_ptr() {
        return format!("*{}", shape(db, target, &inner, depth + 1, seen));
    }
    if let Some((inner, _)) = ty.as_reference() {
        return format!("&{}", shape(db, target, &inner, depth + 1, seen));
    }
    if let Some(adt) = ty.as_adt() {
        let key = ty.display(db, target).to_string();
        if !seen.insert(key.clone()) {
            return "<rec>".to_string();
        }
        let args: Vec<Type> = ty.type_arguments().collect();
        let out = if !args.is_empty() {
            // Generic container (Option/Box/Vec/...): keep the (std, stable) name
            // and decompose the substituted arguments.
            let inner: Vec<String> =
                args.iter().map(|a| shape(db, target, a, depth + 1, seen)).collect();
            format!("{}<{}>", adt.name(db).as_str(), inner.join(","))
        } else {
            match adt {
                Adt::Struct(s) => decompose(db, target, s.fields(db), depth, seen),
                Adt::Union(u) => format!("union{}", decompose(db, target, u.fields(db), depth, seen)),
                Adt::Enum(_) => "enum".to_string(),
            }
        };
        seen.remove(&key);
        return out;
    }
    let args: Vec<Type> = ty.type_arguments().collect();
    if !args.is_empty() {
        let inner: Vec<String> =
            args.iter().map(|a| shape(db, target, a, depth + 1, seen)).collect();
        return format!("<{}>", inner.join(","));
    }
    ty.display(db, target).to_string()
}

fn decompose(
    db: &dyn HirDatabase,
    target: DisplayTarget,
    fields: Vec<Field>,
    depth: usize,
    seen: &mut HashSet<String>,
) -> String {
    let parts: Vec<String> =
        fields.iter().map(|fld| shape(db, target, &fld.ty(db), depth + 1, seen)).collect();
    format!("{{{}}}", parts.join(","))
}
