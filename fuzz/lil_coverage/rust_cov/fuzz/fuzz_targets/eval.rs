#![no_main]
//! Coverage-oriented fuzz of the raw-LLM lil crate. The translation panics/leaks on many
//! inputs; for the COVERAGE question we install a SILENT panic hook (overriding
//! libfuzzer-sys's abort-on-panic hook) + catch_unwind, so panicking inputs are swallowed
//! and the coverage-guided loop keeps exploring. Crash-hunting is a separate run.
use libfuzzer_sys::fuzz_target;
use lil_llm::LilInterpreter;
use std::panic;
use std::sync::Once;

static INIT: Once = Once::new();

fuzz_target!(|data: &[u8]| {
    INIT.call_once(|| { panic::set_hook(Box::new(|_| {})); });
    if let Ok(code) = std::str::from_utf8(data) {
        let code = code.to_string();
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let mut interp = LilInterpreter::new();
            let _ = interp.eval_string(&code);
        }));
    }
});
