//! E2 coverage harness for the raw-LLM lil crate.
//! Drives ONLY the functions the matcher CLAIMED to pair (claimed_pairs.json), to answer:
//! "matching is ~50%, but what crate coverage do we get by fuzzing the paired functions?"
//! Two exercise modes: (a) directly call the 20 PUBLIC paired functions with fuzzed args;
//! (b) feed fuzzed lil programs to eval_string, whose dispatch transitively reaches the 42
//! PRIVATE paired functions (handlers, parser, expr-eval) that no external harness can call.
use lil_llm::{LilInterpreter, LilValue, LilList, SetVarMode, LilCallbacks};
use std::{env, fs};

fn direct_calls(seed: &str) {
    // (a) the 20 public paired targets, exercised directly
    let mut interp = LilInterpreter::new();
    let _ = interp.eval_string(seed);
    let v = LilValue::new(seed.to_string());
    let _ = v.as_str(); let _ = v.is_empty();
    let _ = interp.eval_value(&v);
    let _ = interp.get_var("x");
    let _ = interp.get_var_or("y", LilValue::new("d"));
    interp.set_var("z", LilValue::new(seed), SetVarMode::LocalNew);
    interp.set_error(Some("e")); interp.set_error_at(1, Some("e2"));
    let _ = interp.take_error();
    interp.set_callbacks(LilCallbacks::default());
    interp.register_function("uf", |_i, _a| None);
    let mut l = LilList::new();
    l.push(LilValue::new(seed)); l.push(LilValue::new("b"));
    let _ = l.len(); let _ = l.get(0);
    let _ = l.into_vec();
}

fn main() {
    let path = env::args().nth(1).expect("usage: harness <script.lil>");
    let seed = fs::read_to_string(&path).unwrap_or_default();
    direct_calls(&seed);
    // (b) whole-program dispatch: reaches the private paired handlers/parser
    let mut interp = LilInterpreter::new();
    let _ = interp.eval_string(&seed);
}
