//! Frontend regression tests: arbitrary function names, no benchmark labels or
//! matching scores. Tests the resolved graph, including negative counterexamples.
use std::collections::HashSet;
use std::sync::OnceLock;

fn output() -> &'static analyzer::Output {
    static OUTPUT: OnceLock<analyzer::Output> = OnceLock::new();
    OUTPUT.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("stu-call-graph-test-{}", std::process::id()));
        std::fs::create_dir(&dir).expect("unique test staging directory");
        struct Cleanup(std::path::PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
        }
        let _cleanup = Cleanup(dir.clone());
        std::fs::write(dir.join("Cargo.toml"),
            "[package]\nname=\"graph_fixture\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[lib]\npath=\"lib.rs\"\n[workspace]\n").unwrap();
        std::fs::write(dir.join("lib.rs"), include_str!("fixtures/call_graph.rs")).unwrap();
        analyzer::load_crate(&dir).expect("fixture loads").analyze(true)
    })
}

fn targets(from: &str) -> HashSet<&'static str> {
    let out = output();
    let names: HashSet<_> = out.functions.iter().map(|f| f.name.as_str()).collect();
    out.raw_edges
        .iter()
        .filter(|e| e.from == from && names.contains(e.to.as_str()))
        .map(|e| e.to.as_str())
        .collect()
}

fn check(from: &str, expected: &[&str]) {
    assert_eq!(
        targets(from),
        expected.iter().copied().collect(),
        "caller {from}"
    );
}

#[test]
fn external_homonym_is_not_local_recursion() {
    check("swap", &[]);
    assert!(output()
        .raw_edges
        .iter()
        .any(|e| e.from == "swap" && e.to.starts_with("@nonlocal::")));
}
#[test]
fn direct_recursion_is_preserved() {
    check("direct", &["direct"]);
}
#[test]
fn simple_wrapper_is_not_recursive() {
    check("wrapper", &["leaf"]);
}
#[test]
fn nested_recursion_is_preserved() {
    check("nested_recursive", &["nested_recursive", "leaf"]);
}
#[test]
fn mutual_nested_recursion_is_preserved() {
    check("mutual_helpers", &["mutual_helpers"]);
}
#[test]
fn cycle_through_outer_function_is_preserved() {
    check("outer_cycle", &["outer_cycle"]);
}
#[test]
fn uncalled_nested_definition_does_not_add_edges() {
    check("unused_helper", &["leaf"]);
}
#[test]
fn multiple_nesting_levels_are_projected() {
    check("deep_wrapper", &["leaf"]);
}
#[test]
fn nested_homonym_is_not_a_top_level_call() {
    check("shadowing", &["other"]);
}
#[test]
fn helper_names_in_different_owners_do_not_collide() {
    check("second_wrapper", &["other"]);
}
#[test]
fn module_homonyms_are_both_retained_and_resolved() {
    let left = "graph_fixture::left::twin";
    let right = "graph_fixture::right::twin";
    check("invoke_left", &[left]);
    check("invoke_right", &[right]);
    check(left, &["leaf"]);
    check(right, &["other"]);
}
#[test]
fn methods_and_their_nested_helpers_resolve() {
    check("use_method", &["Boxed::method", "Boxed::wrapped_method"]);
    check("Boxed::method", &["leaf"]);
    check("Boxed::wrapped_method", &["other"]);
}
#[test]
fn excluded_owners_do_not_leak_nested_bodies() {
    assert!(!output()
        .functions
        .iter()
        .any(|f| f.name.contains("default") || f.name.contains("not_a_candidate")));
    assert!(!output()
        .raw_edges
        .iter()
        .any(|e| e.from.contains("default") || e.from.contains("not_a_candidate")));
}
#[test]
fn indirect_calls_stay_unresolved() {
    check("indirect", &[]);
    assert!(output().indirect_calls.iter().any(|e| e.from == "indirect"));
}
#[test]
fn bodyless_foreign_homonym_is_not_local() {
    check("foreign_call", &[]);
}
#[test]
fn emitted_candidate_ids_are_unique_and_helpers_are_excluded() {
    let out = output();
    let names: HashSet<_> = out.functions.iter().map(|f| &f.name).collect();
    assert_eq!(names.len(), out.functions.len());
    assert!(!out.functions.iter().any(
        |f| ["helper", "inner", "hidden", "level_one", "level_two"].contains(&f.name.as_str())
    ));
    assert!(out
        .excluded_scaffolding
        .iter()
        .any(|f| f.reason == "nested_local"));
}

#[test]
fn specialized_impl_homonyms_remain_distinct() {
    let a = targets("use_u8");
    let b = targets("use_u16");
    assert_eq!(a.len(), 1);
    assert_eq!(b.len(), 1);
    assert_ne!(a, b);
    check(a.into_iter().next().unwrap(), &["leaf"]);
    check(b.into_iter().next().unwrap(), &["other"]);
}

#[test]
fn explicit_unmangled_export_resolves_across_extern_declaration() {
    check("linked_call", &["published"]);
}
#[test]
fn link_name_uses_export_symbol_not_rust_spelling() {
    check("linked_alias", &["actual_name"]);
}
#[test]
fn c_abi_without_an_export_contract_is_not_enough() {
    check("unlinked_mangled_call", &[]);
}
#[test]
fn recursion_through_link_alias_is_preserved() {
    check("exported_recursion", &["exported_recursion"]);
}
