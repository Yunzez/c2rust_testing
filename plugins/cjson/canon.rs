// Comparator plugin for cJSON — the Rust half.
//
// STABLE ABI (docs/harness_oracle_plan.md §5):
//     unsafe fn c2r_canon_rust(obj: *const core::ffi::c_void) -> Vec<u8>
// Returns the canonical byte form of *obj, byte-identical to what the C half writes.
//
// The translated crate is always in scope as `translated`, so this file names no crate. That is
// NOT the same as being translator-independent: it reads the translated struct's fields by name,
// so it is reusable exactly across translations whose layout and field names stay compatible with
// the C header's. A translator that reshapes cJSON into an idiomatic enum needs its own
// comparator, and that is a property of the plugin, not a limitation of the interface.
//
// Produced once by tools/stu_selector/contract_templates.py and now owned as source.
fn c2r_r_int(o: &mut Vec<u8>, v: i64) { o.extend_from_slice(format!("i{v};").as_bytes()); }
fn c2r_r_dbl(o: &mut Vec<u8>, u: u64) { o.extend_from_slice(format!("d{u:016x};").as_bytes()); }
unsafe fn c2r_r_str(o: &mut Vec<u8>, s: *const u8) {
    if s.is_null() { o.extend_from_slice(b"sN;"); return; }
    let mut n = 0usize; while *s.add(n) != 0 { n += 1; }
    o.extend_from_slice(format!("s{n}:").as_bytes());
    for i in 0..n { o.extend_from_slice(format!("{:02x}", *s.add(i)).as_bytes()); }
    o.push(b';');
}
unsafe fn c2r_r_node(o: &mut Vec<u8>, p: *const translated::cJSON, depth: u32,
                     seen: &mut Vec<usize>, nodes: &mut usize) {
    if p.is_null() { o.push(b'N'); return; }
    if depth > 64 || *nodes >= 4096 || seen.contains(&(p as usize)) { o.push(b'X'); return; }
    if seen.len() < 4096 { seen.push(p as usize); }
    *nodes += 1;
    o.push(b'(');
        c2r_r_int(o, (*p).type_0 as i64);
        c2r_r_int(o, (*p).valueint as i64);
        c2r_r_dbl(o, ((*p).valuedouble as f64).to_bits());
        c2r_r_str(o, (*p).valuestring as *const u8);
        c2r_r_str(o, (*p).string as *const u8);
    o.push(b'[');
    let mut q = (*p).child as *const translated::cJSON;
    while !q.is_null() {
        if *nodes >= 4096 { o.push(b'X'); break; }
        c2r_r_node(o, q, depth + 1, seen, nodes);
        q = (*q).next as *const translated::cJSON;
    }
    o.push(b']');
    o.push(b')');
}
unsafe fn c2r_r_extract(p: *const translated::cJSON) -> Vec<u8> {
    let (mut o, mut seen, mut nodes) = (Vec::new(), Vec::new(), 0usize);
    c2r_r_node(&mut o, p, 0, &mut seen, &mut nodes);
    o
}


// ---- stable ABI entry point ------------------------------------------------------------------
pub unsafe fn c2r_canon_rust(obj: *const core::ffi::c_void) -> Vec<u8> {
    c2r_r_extract(obj as *const _)
}
