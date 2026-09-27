// Representation-only adapter: the shipped bzip2 acceptance suite (Makefile `test:`) drives
// the `bzip2` CLI.  The translated crate contains the transpiled CLI (bzip2.rs, `pub fn main`)
// but never declares it as a binary.  This wrapper is the whole adapter — it adds no logic.
fn main() {
    bz_cov::bzip2::main()
}
