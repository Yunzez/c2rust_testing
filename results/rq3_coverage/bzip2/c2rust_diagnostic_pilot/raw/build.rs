use std::process::Command;

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=shims.c");
    let st = Command::new("cc")
        .args(["-O1", "-fPIC", "-c", "shims.c", "-o", &format!("{out}/shims.o")])
        .status()
        .expect("cc");
    assert!(st.success());
    let st = Command::new("ar")
        .args(["rcs", &format!("{out}/libshims.a"), &format!("{out}/shims.o")])
        .status()
        .expect("ar");
    assert!(st.success());
    println!("cargo:rustc-link-search=native={out}");
    println!("cargo:rustc-link-lib=static=shims");
}
