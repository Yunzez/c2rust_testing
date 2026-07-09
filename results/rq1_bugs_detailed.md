# RQ1 — The 20 Bugs, In Detail (C code · Rust code · what exactly broke)

**Companion to `rq1_master_table.md` (Table 1).** Ledger: **7 crash + 13 semantic = 20 confirmed
bugs** across 5 published tools (C2SaferRust, Laertes, CROWN, SACTOR, PtrTrans). Every bug is
C-backed: differential execution vs the original C, ASan/UBSan gate on the C side (divergence counts
only on UB-free inputs), same-source attribution (the mechanical c2rust baseline is verified ≡ C
first, so the defect is localized to the tool's rewrite).

**★ = worth a deep read** (headline-class; each can carry a standalone case study).

Sourcing convention: all code below is quoted **verbatim from the archives / shipped translations**
(paths cited per bug). Where an original `.c` is not archived, the faithful base-c2rust Rust is the
reference side (it is verified byte-exact ≡ C by the differential itself) and that is stated inline.

### Class index (recurring mechanisms)

| class | bugs | one-liner |
|---|---|---|
| **zeroed-table corruption** | S1 S3 S4 S5 (4×, 3 tools) | a lookup table that silently stays zero — three distinct mechanisms, one symptom |
| **UTF-8 panic** | C2 C3 C4 C5 (4×, 1 tool) | `to_str().unwrap()` where C handled raw bytes |
| **NULL/empty conflation** | S1 S2 S12 | ptr→slice lift rewrites `p == NULL` as `.is_empty()` |
| **reshaping-contract loss** | S6 S7 S8 S9 (PtrTrans) | callee faithful in isolation; the reshaped call contract is wrong |
| **ownership-lift breakage** | C7 S10 S11 (CROWN) | the *safety* lifter introduces memory corruption |
| **driver/guard rewrite** | C1 C6 S13 | "idiomatic" reordering / index-type change breaks a C invariant |

---
---

# Part I — Crash bugs (7)

---

## C1 ★ qsort × C2SaferRust — `int→usize` breaks the recursion-termination sentinel

**Class:** crash (infinite recursion → OOB read, ASan abort) · **driver/guard rewrite**
**Evidence:** `results/rq1_bugs/qsort_c2saferrust/`

**Mechanism.** C `quickSort` terminates its recursion through a *negative sentinel*: when
`partition` returns 0, the left recursion is called with `high = -1` and `low(0) < -1` is false.
C2SaferRust's safety-lift rewrote the indices to `usize` and `i-1` to `i.wrapping_sub(1)` — when
`i == 0` that is `usize::MAX`, `low < usize::MAX` is always true, and the recursion never terminates.
En route `usize::MAX as i32 == -1` flows back into `partition`, which dereferences `*arr.offset(-1)`
— an out-of-bounds read.

**C** (`source/qsort.c`):
```c
void quickSort(int arr[], int low, int high)
{
    if (low < high) {
        int i = partition(arr, low, high);
        quickSort(arr, low, i - 1);      /* i-1 can be -1; then low(0) < -1 is FALSE -> stop */
        quickSort(arr, i + 1, high);
    }
}
```

**Rust** (`translated/c2saferrust_WIP.rs`):
```rust
pub fn quickSort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let i = unsafe { partition(arr.as_mut_ptr(), low as i32, high as i32) } as usize;
        quickSort(arr, low, i.wrapping_sub(1));   // <-- BUG: i==0 -> usize::MAX, never < -> infinite recursion
        quickSort(arr, i + 1, high);
    }
}
```

**The defect.** `i - 1` on a signed int (can legally be −1, the loop sentinel) became
`i.wrapping_sub(1)` on `usize` (wraps to `usize::MAX`). The "make it safer" type rewrite silently
deleted the termination invariant the C code encoded in signedness.

**Trigger + numbers.** libFuzzer crashed at **~81 executions, <1 s**; minimized to a **2-byte input
= array `[5, 0]`** (any array where the pivot is the minimum). ASan: heap-buffer-overflow READ of
size 4 (the `*arr.offset(-1)`). Cross-tool contrast on the same 30 LOC: Laertes kept `i32` (`i - 1`)
→ certificate 50,827 records 0 diffs; CROWN ditto; **PtrTrans doesn't sort at all (S6)** — 3 tools,
3 outcomes.

---

## C2 — urlparser × C2SaferRust — `url_is_ssh` panics on non-UTF-8 (UTF-8-panic class)

**Class:** crash (panic) · **UTF-8 panic**
**Evidence:** `results/rq1_bugs/utf8_panic_c2saferrust/` (Instance A); code from shipped `urlparser_WIP/`

**Mechanism.** C compares the protocol string byte-wise with `strcmp` — arbitrary bytes are fine,
non-matches return `false`. The lift routes the same bytes through
`CStr::from_ptr(...).to_str().unwrap()`, which panics on the first non-UTF-8 byte.

**C** (`laertes_benchmarks/urlparser_WIP/url.h:338`):
```c
bool
url_is_ssh (char *str) {
  str = strdup(str);
  if (0 == strcmp(str, "ssh") ||
      0 == strcmp(str, "git")) {
    free(str);
    return true;
  }
  return false;
}
```

**Rust** (`urlparser_WIP/test.rs:484`):
```rust
pub fn url_is_ssh(str: *mut std::os::raw::c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(str) };
    let str_slice = c_str.to_str().unwrap();   // <-- BUG: panics on any non-UTF-8 byte
    str_slice == "ssh" || str_slice == "git"
}
```

**The defect.** C never fails on this path (returns a bool for any bytes); the Rust translation
aborts the process. Not semantics-preserving: a total function became partial.

**Trigger.** Fuzz bytes `31 72 8e` → protocol containing `0x8e`: C/base returns `false` (rc 0);
WIP panics at `test.rs:486`.

**Class note.** Site census across the 6 C2SaferRust programs: **~27 `to_str()/from_utf8().unwrap()`
sites**, 4 confirmed library-reachable instances (C2 C3 C4 C5 — protocol string, `system` arg,
filename, directory name). genann's 4 sites are constant-string (never panic) — the census is an
upper bound, the 4 confirmed instances are the class.

---

## C3 — lil × C2SaferRust — `do_system` panics on non-UTF-8 argv (UTF-8-panic class)

**Class:** crash (panic) · **UTF-8 panic**
**Evidence:** `results/rq1_bugs/utf8_panic_c2saferrust/lil_do_system_diff.rs`; code from shipped `lil_WIP/`

**Mechanism.** lil's `system` builtin concatenates its argv into a shell command; C uses
`strlen`+`memcpy` on raw bytes. The rewrite pushes each arg through `to_str().unwrap()`.

**C** (`lil_WIP/main.c:47`):
```c
static char* do_system(size_t argc, char** argv)
{
    ...
    for (i=0; i<argc; i++) {
        size_t len = strlen(argv[i]);
        ...
        cmd = realloc(cmd, cmdlen + len);
        memcpy(cmd + cmdlen, argv[i], len);   /* raw bytes, no validation */
        cmdlen += len;
    }
```

**Rust** (`lil_WIP/main.rs:149`):
```rust
fn do_system(argc: usize, argv: *mut *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let mut cmd = String::new();
    let mut i: usize = 0;
    while i < argc {
        let arg = unsafe { std::ffi::CStr::from_ptr(*argv.offset(i as isize)) };
        if i != 0 {
            cmd.push(' ');
        }
        cmd.push_str(arg.to_str().unwrap());   // <-- BUG: panics on non-UTF-8 argument
        i += 1;
    }
```

**The defect.** `arg.to_str().unwrap()` panics where C's `memcpy` copies verbatim. (Same function
also carries `String::from_utf8(output.stdout).expect(...)` at main.rs:168 — a second panic site on
the subprocess *output*; the argv site is the documented instance.)

**Trigger.** A lil script passing `system` an argument containing bytes like `0x8e 0x8f` → C builds
the command; WIP panics.

---

## C4 — bzip2 × C2SaferRust — `endsInBz2` panics on non-UTF-8 filename (UTF-8-panic class)

**Class:** crash (panic) · **UTF-8 panic**
**Evidence:** no dedicated archive dir; Rust from shipped `bzip2_WIP/bzip2recover.rs`, C from `crown/c-code/bzip2/bzip2recover.c`; repro `results/rq1_bugs/utf8_panic_c2saferrust/bzip2_endsInBz2_diff.rs`

**Mechanism.** `endsInBz2` checks whether a filename ends in `.bz2`. Linux filenames are arbitrary
bytes; C does byte-wise comparison. The rewrite validates UTF-8 first — and panics.

**C** (`crown/c-code/bzip2/bzip2recover.c:261`):
```c
static Bool endsInBz2 ( Char* name )
{
   Int32 n = strlen ( name );
   if (n <= 4) return False;
   return
      (name[n-4] == '.' &&
       name[n-3] == 'b' &&
       name[n-2] == 'z' &&
       name[n-1] == '2');
}
```

**Rust** (`bzip2_WIP/bzip2recover.rs:274`):
```rust
fn endsInBz2(name: *mut std::os::raw::c_char) -> i32 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(name) };
    let str_slice = c_str.to_str().unwrap();   // <-- BUG: panics on non-UTF-8 filename
    let n = str_slice.len();
    if n <= 4 { return 0; }
    if str_slice.ends_with(".bz2") { return 1; }
    0
}
```

**The defect.** A legitimately-named `.bz2` file whose name contains one non-UTF-8 byte crashes
`bzip2recover` before it does anything; C answers 0/1 for any bytes.

**Trigger.** Filename bytes `[0x8e, '.', 'b', 'z', '2']` → C/base returns 1; WIP panics.

---

## C5 — optipng × C2SaferRust — `-dir` path panics on non-UTF-8 directory (UTF-8-panic class)

**Class:** crash (panic) · **UTF-8 panic**
**Evidence:** shipped `optipng_WIP/src/optipng/optim.rs:3096` (buggy) vs base `optipng/src/optipng/optim.rs:3238` (faithful); class doc `results/rq1_bug_table.md`

**Mechanism.** optipng's `-dir <name>` option stores the output directory as a C string; the write
path calls `opng_os_create_dir(options.dir_name)`. The lift interposes `to_str().unwrap()` on the
user-controlled path.

**Rust — base c2rust, faithful** (`optipng/src/optipng/optim.rs:3238`; original `.c` not in repo,
base is the verified-faithful reference):
```rust
            opng_os_create_dir(options.dir_name);            // raw C pointer straight through
```

**Rust — C2SaferRust WIP, buggy** (`optipng_WIP/src/optipng/optim.rs:3096`):
```rust
        if !options.dir_name.is_null() {
            opng_os_create_dir(std::ffi::CStr::from_ptr(options.dir_name).to_str().unwrap());
            //                                                            ^^^^^^^^^^^^^^^^^ BUG: panics on non-UTF-8 dir name
        }
```

**The defect.** `optipng -dir <non-UTF-8 name> file.png` — a valid invocation on Linux, where paths
are bytes — panics in the WIP where C (and base) create the directory. 12 `unwrap` sites exist in
optipng_WIP; this is the confirmed input-reachable one.

---

## C6 — tulipindicators × C2SaferRust — guard hoisting: `argv[1]` read before the `argc` check

**Class:** crash (SIGSEGV, NULL deref) · **driver/guard rewrite**
**Evidence:** `results/rq1_bugs/tulip_c2saferrust/` (Bug 1, `excerpt_guard_hoisting.rs`)

**Mechanism.** C checks `argc < 2` *before* touching `argv[1]`. The "idiomatic" rewrite hoisted the
`argv[1]` read and the indicator lookup *above* the guard. With zero CLI args, `argv[1]` is the
C-guaranteed NULL terminator → `CStr::from_ptr(NULL)` → SIGSEGV.

**Base c2rust — faithful, guard first** (`excerpt_base_faithful.rs`; tulip C source not in repo,
base is the verified-faithful reference):
```rust
    let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    if argc < 2 as std::os::raw::c_int {
        printf(b"No indicator given.\n\x00" as *const u8 as *const std::os::raw::c_char);
        ...
        return 1 as std::os::raw::c_int
    }
    if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),   /* argv[1] read AFTER the guard */
```

**Rust — C2SaferRust WIP, buggy** (`tulipindicators_WIP/sample.rs:108`):
```rust
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let argv1 = unsafe { CStr::from_ptr(*argv.offset(1)).to_str().unwrap() };   // <-- BUG: argv[1] read BEFORE the guard
let info = ti_indicators.iter().find(|indicator| unsafe { CStr::from_ptr(indicator.name).to_str().unwrap() } == argv1);
if argc < 2 {
    println!("No indicator given.");
    ...
    return 1;
}
```

**The defect.** The dereference moved above the bounds check that makes it legal — a control-
dependence the rewrite did not preserve.

**Trigger.** `./sample` with no arguments → base prints usage, exit 1; WIP → SIGSEGV (exit 139).

---

## C7 — bzip2 × CROWN — heap corruption during compress

**Class:** crash (glibc abort) · **ownership-lift breakage** · part of headline #3 (with S10/S11)
**Evidence:** `results/rq1_bugs/bzip2_crown/` — *behavioral attribution; the archive contains no
line-level CROWN-vs-base code diff (scratch was on wiped /tmp), so no specific Rust lines are cited
as "the" defect. Attribution is by same-source differential: C oracle ASan/UBSan-clean, base c2rust
byte-exact ≡ C, CROWN diverges.*

**Mechanism.** CROWN's ownership/slice rewrite of the `BZ2_bzBuffToBuffCompress` path writes past a
heap allocation on ~25% of inputs; glibc aborts with `free(): invalid next size (normal)`. The
mechanical baseline it rewrote is clean on the same inputs — the *safety* lift introduced the memory
unsafety.

**C — the entry point exercised** (`oracle_comp.c:15`):
```c
int rc=BZ2_bzBuffToBuffCompress((char*)out,&dl,(char*)in,len,par[0],0,par[1]);
```

**Rust — the reproducing call** (`crown_compress_driver.rs:14`; per-record process isolation because
the call can abort the batch):
```rust
        unsafe{
            let mut dl:u32=out.len() as u32;
            let rc=BZ2_bzBuffToBuffCompress(out.as_mut_ptr() as *mut i8,Some(&mut dl),
                src.as_ptr() as *mut i8,len as u32,bs,0,wf);
```

**Numbers.** 150-record sample: **38/150 ≈ 25% abort** with heap-metadata corruption; C oracle 0
sanitizer reports on the full corpus. See S10/S11 for the sibling semantic breakage; the full CROWN
split is **29% correct / 46% silently-corrupt / 25% crash**.

---
---

# Part II — Semantic bugs (13)

*The class the project exists for: no crash, no error code — only a C-side differential sees them.*

---

## S1 ★ optipng(zlib) × C2SaferRust — `crc32_z` empty-chunk resets the running CRC (headline #1)

**Class:** semantic · **NULL/empty conflation** · zeroed-table family (instance 1 of 4)
**Evidence:** `results/rq1_bugs/crc32_c2saferrust/`

**Mechanism.** zlib's incremental `crc32_z(crc, buf, len)` returns 0 only when the *pointer* is
NULL (the "give me the initial value" convention). C2SaferRust lifted `buf: *const u8` to
`buf: &[u8]` and rewrote the NULL guard as `is_empty()`. A NULL pointer and a valid zero-length
chunk are different things: on an empty chunk zlib returns the running `crc` unchanged; the lift
returns 0 — silently resetting any streaming CRC that ever sees a zero-length chunk.

**Reference — base c2rust, faithful** (`source_excerpts.txt`, from `optipng/src/zlib/crc32.rs:2071`;
original `.c` not archived, base is the verified-faithful reference):
```rust
pub unsafe extern "C" fn crc32_z(mut crc: std::os::raw::c_ulong,
                                 mut buf: *const std::os::raw::c_uchar,
                                 mut len: z_size_t) -> uLong {
    if buf.is_null() { return 0 as std::os::raw::c_ulong }
```

**Rust — C2SaferRust WIP, buggy** (`optipng_WIP/src/zlib/crc32.rs:2084`):
```rust
pub fn crc32_z(crc: u64, buf: &[u8], len: usize) -> u64 {
    if buf.is_empty() { return 0 }    // <-- BUG: true for every zero-length slice, not just NULL
```

**The defect.** `is_null()` (pointer identity) became `is_empty()` (length). The two coincide only
when the caller never passes a legitimate empty buffer — exactly what streaming APIs do all the time.

**Trigger + numbers.** `crc32_z(0x12345678, "", 0)` → base `0x12345678`, WIP `0x00000000`.
**1,000,000-trial sweep: 1,664 divergences — every single one at `len==0`**; all non-empty buffers
agree exactly (that's what makes it invisible to output-crash fuzzing: nothing ever fails, the
checksum is just wrong). Three-way: Laertes returns `0xdeadbeef` correctly on the same call. End-to-
end: segmented IDAT `[5,0,5]` → base CRC `0xe221bc33`, WIP `0xb2a113e5` — a wrong CRC written into a
PNG. C2SaferRust's own log marked this chunk `Success`.

---

## S2 — optipng(zlib) × C2SaferRust — `adler32_z` empty-chunk reset (sibling of S1)

**Class:** semantic · **NULL/empty conflation**
**Evidence:** `results/rq1_bugs/crc32_c2saferrust/adler32_corroboration/`

**Mechanism.** Same crate, same lift, zlib's other checksum: the NULL guard `if buf.is_null()
{ return 1 }` (Adler seed) became a length test.

**Reference — base** (`adler32_corroboration/src/base.rs:45`):
```rust
    /* initial Adler-32 value (deferred check for len == 1 speed) */
    if buf.is_null() { return 1 as std::os::raw::c_long as uLong }
```

**Rust — WIP, buggy** (`adler32_corroboration/src/wip.rs:46`):
```rust
/* initial Adler-32 value (deferred check for len == 1 speed) */
if len == 0 { return 1; }    // <-- BUG: resets running Adler-32 to seed on any empty chunk
```

**Trigger + numbers.** `adler32_z(0x12345678, buf, 0)` → base `0x12345678`, WIP `0x00000001`.
**Honesty caveat (recorded in the archive):** unlike crc32_z, adler32_z was marked `Failure` in
C2SaferRust's own log and is *additionally* miscompiled on non-empty input (wrong sums + an OOB
panic in its `from_raw_parts(buf, 16)` fast path). It corroborates the conflation class; **S1 is the
clean headline** (tool said Success, output silently wrong).

---

## S3 ★ bzip2 × Laertes — uncalled table-init zeroes `BZ2_crc32Table` → 91% integrity-invalid streams (headline #4)

**Class:** semantic · **zeroed-table** (instance 2; the funny mechanism)
**Evidence:** `results/rq1_bugs/bzip2_laertes/`

**Mechanism.** Laertes lowers C compile-time const-array initializers into a runtime init function —
and emits **zero call sites for it**. `BZ2_crc32Table` (256 precomputed constants in C) becomes an
all-zero static plus a defined-but-never-called `laertes_init_BZ2_crc32Table()`. The CRC recurrence
`blockCRC = (blockCRC<<8) ^ table[(blockCRC>>24)^byte]` loses its table term; compression still
returns `BZ_OK` and writes a wrong CRC into every stream.

**C.** In C the table is a compile-time constant initializer (`const UInt32 BZ2_crc32Table[256] =
{ 0x00000000, 0x04c11db7, 0x09823b6e, ... };` — original `.c` not archived; the faithful base
c2rust carries the same 256 real values as a static initializer).

**Rust — Laertes, buggy** (`crctable_zeroed_static.rs`):
```rust
pub static mut BZ2_crc32Table: [std::os::raw::c_uint; 256] =
    [0,0,0,0,0,0,0,0, /* ...all 256 zeros... */ 0,];     // <-- what the program actually reads

unsafe fn laertes_init_BZ2_crc32Table() {                 // <-- holds the real values...
BZ2_crc32Table = [0 as std::os::raw::c_long as UInt32, 0x4c11db7 as std::os::raw::c_long as UInt32,
     0x9823b6e as std::os::raw::c_long as UInt32, 0xd4326d9 as std::os::raw::c_long as UInt32,
     0x130476dc as std::os::raw::c_long as UInt32,
     /* ... */
}   // ...and is NEVER CALLED: 38 laertes_init_* functions in the crate, total call sites: 0
```

**The defect.** The real initializer exists in the binary — inside a function nothing invokes. The
table term of the CRC recurrence is permanently 0.

**Trigger + numbers.** Compress `"A"` → C stream carries CRC bytes `19 93 9b 6b`; Laertes carries
`00 00 00 ff` (Huffman payload byte-identical — *only* the CRC differs; both archived, hexdump-
confirmed). Oracle: canonical `bunzip2 -t` over 100 records → **9 valid / 91 CRC-error = 91%
integrity-invalid**, with `BZ_OK` returned every time. Base c2rust: byte-identical to C.

---

## S4 ★ optipng(zlib) × Laertes — same uncalled-init, zlib `crc_table`, no rebuild path → 98.49% wrong (3rd checksum instance)

**Class:** semantic · **zeroed-table** (instance 3)
**Evidence:** `results/rq1_bugs/optipng_laertes/`

**Mechanism.** Same Laertes mechanism as S3, new victim: optipng's bundled zlib is compiled
**without `DYNAMIC_CRC_TABLE`**, so `crc_table` is a precomputed static read directly by `crc32_z` —
there is no `make_crc_table` runtime-rebuild guard to save it. Laertes zeroes it the same way.

**Rust — Laertes, buggy** (`evidence_zeroed_crc_table.txt` + `src/laertes.rs:14`):
```rust
static mut crc_table: [[std::os::raw::c_uint; 256]; 8] =
    [[0,0,0, /* ...all 8x256 entries zero... */ ]];       // <-- read directly by crc32_z

unsafe fn laertes_init_crc_table() {                       // <-- real values (0x77073096, ...), never called
crc_table = [[0 as std::os::raw::c_ulong as z_crc_t, 0x77073096 as std::os::raw::c_ulong as z_crc_t,

pub unsafe extern "C" fn get_crc_table() -> * const std::os::raw::c_uint {
    /* DYNAMIC_CRC_TABLE */                                // <-- the rebuild path survives only as a comment
    return crc_table.as_ptr() as *const z_crc_t;
}
```

**The defect.** Identical to S3, but here the *library configuration* (no DYNAMIC_CRC_TABLE) means
there is no runtime path that could ever repopulate the table — the zeroing is unconditionally fatal.

**Trigger + numbers.** `crc32("a")` C=`e8b7be43`, Laertes=`ff000000`; `crc32("hello")` C=`3610a686`,
Laertes=`ffffffff`. Corpus 200,006 strings: **196,985 wrong = 98.49%**. Control: `adler32` (arithmetic,
no table) — **0 diffs**, cleanly isolating the bug to the table path. C oracle ASan/UBSan: 0 reports.

**The nuance that defends against overclaim** (footnote 16): genann's sigmoid `lookup` is *also*
Laertes-zeroed with an uncalled init — and it's **harmless** there, because
`genann_act_sigmoid_cached` retains its lazy rebuild (`if !initialized { build }`). The uncalled-init
bug is fatal exactly when the zeroed static is the sole value source (S3, S4) and benign when a
runtime rebuild exists. Mechanism-specific, not blanket "Laertes zeroes tables."

---

## S5 ★ genann × SACTOR — the immutable lookup table: writes are UB'd away, 100.00% divergence (headline #6)

**Class:** semantic (silent all-zero output; debug build SIGSEGVs) · **zeroed-table** (instance 4 — a *third* distinct mechanism)
**Evidence:** `results/rq1_bugs/genann_sactor/` (`verbatim_lookup_global.rs`, `verbatim_init_writer.rs`, `assembled_translation.rs`)

**Mechanism.** C declares `double lookup[4096]` as a mutable file-scope array, populated at runtime
by `genann_init_sigmoid_lookup` (which *is* called). SACTOR translated it as an **immutable** Rust
`static` — and made the init write through a `*const → *mut` cast, which is **Rust UB**. Release
LLVM assumes the static never mutates and folds every read to `0.0`: every activation returns 0,
`genann_run` outputs all zeros, silently. Debug builds SIGSEGV (the write actually hits `.rodata`) —
proving the UB.

**C** (`crown/c-code/genann-1.0.0/c/genann.c`):
```c
#define LOOKUP_SIZE 4096
double interval;
double lookup[LOOKUP_SIZE];                       /* mutable file-scope array */

void genann_init_sigmoid_lookup(const genann *ann) {
        const double f = (sigmoid_dom_max - sigmoid_dom_min) / LOOKUP_SIZE;
        int i;
        interval = LOOKUP_SIZE / (sigmoid_dom_max - sigmoid_dom_min);
        for (i = 0; i < LOOKUP_SIZE; ++i) {
            lookup[i] = genann_act_sigmoid(ann, sigmoid_dom_min + f * i);
        }
}

double genann_act_sigmoid_cached(const genann *ann unused, double a) {
    ...
    return lookup[j];
}
```

**Rust — SACTOR, buggy** (verbatim output):
```rust
static lookup: [f64; 4096] = [0.0; 4096];          // <-- BUG root: immutable (C's array is mutable)

pub unsafe fn genann_init_sigmoid_lookup(ann: *const genann) {
    let f: f64 = (sigmoid_dom_max - sigmoid_dom_min) / 4096.0;
    interval = 4096.0 / (sigmoid_dom_max - sigmoid_dom_min);
    let mut i: libc::c_int = 0;
    while i < 4096 {
        let ptr = &lookup as *const [f64; 4096] as *mut [f64; 4096];   // <-- const-cast write = Rust UB
        (*ptr)[i as usize] = genann_act_sigmoid(ann, sigmoid_dom_min + f * (i as f64));
        i += 1;
    }
}
```
The reads (`assembled_translation.rs:93`) go straight to `crate::lookup[j]`. Contrast: the sibling
global `interval` **was** translated mutable (`pub static mut interval: f64 = 0.0;`) — only `lookup`
lost its mutability.

**The defect.** Mutability loss on one global. The init *runs* and its writes are semantically
erased by the optimizer, because writing an immutable static through a cast pointer is UB. Unlike
S3 (init never called), here everything executes — and still the table stays zero.

**Trigger + numbers.** **5,000/5,000 = 100.00% divergence** on random inputs; 12/12 of genann's own
embedded tests diverge; release exit 0 / no panic / all-zero network output; debug: 100% SIGSEGV
(exit 139, `.rodata` write). **The method finding:** SACTOR's per-function verification embeds each
translated fn into the *C* program via FFI — where `lookup` is C's mutable array — so **all 15
functions passed its own tests**; the bug exists only in the all-Rust whole. Per-function
verification ≢ whole-program correctness.

**Zeroed-table family complete:** same genann table Laertes attacks harmlessly (lazy rebuild saves
it); SACTOR's rebuild runs and the writes vanish. Four instances, three mechanisms (uncalled init /
NULL-empty conflation / mutability loss), one symptom: a lookup table that silently stays zero.

---

## S6 ★ qsort × PtrTrans — the sort that doesn't sort: `split_at_mut` wrong index + None-swallowing swap (headline #5)

**Class:** semantic (68% of arrays come back unsorted; zero panics) · **reshaping-contract loss**
**Evidence:** `results/rq1_bugs/qsort_ptrtrans/`

**Mechanism.** PtrTrans reshaped `int arr[]` → `Option<&mut [i32]>`. Two `&mut` into one slice is
illegal, so it rewrote C's `swap(&arr[i], &arr[j])` via `split_at_mut(max(i,j))` — after which
element `j` lives at `right[0]`. The generated code indexes `right.get_mut(j - i)` instead: element
`2j − i`. And its defensively-designed `swap(Option, Option)` **no-ops on None**, so every
out-of-range wrong index is silently swallowed. The crate compiles, passes PtrTrans's own cargo
gate, and never panics.

**C** (`original_qsort.c:8`):
```c
int partition (int arr[], int low, int high)
{
    int pivot = arr[high];
    int i = low - 1;

    for (int j = low; j <= high - 1; j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return i + 1;
}
```

**Rust — PtrTrans, buggy** (`translated_qsort.rs:47`):
```rust
                let (left, right) = arr.split_at_mut(j_usize.max(i_usize));
                // Obtain mutable references to the two elements without aliasing.
                let a_ref: Option<&mut i32>;
                let b_ref: Option<&mut i32>;
                if i_usize <= j_usize {
                    a_ref = left.get_mut(i_usize);
                    b_ref = right.get_mut(j_usize - i_usize); // <-- BUG: element 2j-i; correct is right.get_mut(0)
                } else {
                    a_ref = right.get_mut(i_usize - j_usize);
                    b_ref = left.get_mut(j_usize);
                }
                swap(a_ref, b_ref);
```
The post-loop `swap(&arr[i+1], &arr[high])` repeats the identical error (`translated_qsort.rs:67`:
`b_ref = right.get_mut(high_usize - ip1_usize)` = element `2·high − (i+1)`).

**The defect.** After `split_at_mut(j)` (with `i ≤ j`), the second element is `right[0]`; the code
computes `right[j-i]`. Wrong element gets swapped — or nothing does, because the `Option`-based swap
silences the out-of-range case that would have panicked and exposed the bug. The safety idiom is
what makes the bug *silent*.

**Trigger + numbers.** Minimal repro `[3,1,2,5,4]` → C `1 2 3 4 5`, PtrTrans `2 5 3 1 4`. Batch:
**34,012/50,000 = 68% of UB-free random arrays return UNSORTED** (every diverging output fails the
sortedness check; the agreeing 32% are trivially small). Same reshaping-contract class as S7-S9,
distilled to 30 LOC. Cross-tool: the same function C2SaferRust crashes on (C1), Laertes/CROWN
certify clean.

---

## S7 ★ cJSON × PtrTrans — `\u` escapes dead: empty-slice `input_end` fabricated at the call site (headline #2, part 1/3)

**Class:** semantic · **reshaping-contract loss** (call-site, not callee)
**Evidence:** `results/rq1_bugs/cjson_ptrtrans/` (`excerpt_utf16_gate.rs`, `excerpt_callsite_and_valuestring.rs`, `translated_crate/src/cjson.rs`)

**Mechanism.** C's `utf16_literal_to_utf8(first_sequence, input_end, ...)` takes a *bound pointer*
and rejects short input via `(input_end - first_sequence) < 6`. PtrTrans lifted both pointers to
slices and modeled the bound check as `input_end.len() < 6` — correct *if* the caller passes a slice
spanning the remaining input. But the caller in `parse_string` fabricates `input_end` as an **empty
slice** (constructed at a single index, purely to satisfy the signature). `len()` is always 0 < 6 →
every `\uXXXX` escape returns 0 (parse error). The callee is faithful given consistent slices —
standalone differential: 0 diffs. The defect lives entirely in the reshaped call contract.

**C contract** (modern cJSON.c not archived; the oracle calls it with a real end pointer —
`oracle.c:30`):
```c
        } else if (op==3) { /* utf16_literal_to_utf8 */
            unsigned char out[64]; unsigned char*po=out;
            unsigned char r = utf16_literal_to_utf8(pay, pay+len, &po);   /* input_end = REAL bound */
```

**Rust — callee gate (faithful in isolation)** (`translated_crate/src/cjson.rs:166`):
```rust
    // (input_end - first_sequence) < 6  -->  remaining input less than 6 bytes
    if input_end.len() < 6 {
        return 0;
    }
```

**Rust — call site (the bug)** (`translated_crate/src/cjson.rs:823`):
```rust
                    let input_slice = &content[input_pointer_index..input_end_index];
                    let end_slice = &content[input_end_index..input_end_index]; // <-- BUG: empty slice => len()==0 < 6 always
                    sequence_length = utf16_literal_to_utf8(
                        Some(input_slice),
                        Some(end_slice),
                        Some(&mut output),
                    );
```

**The defect.** `end_slice` is `content[i..i]` — length 0 by construction. The callee's (correct)
short-input guard fires on every call. Every unicode escape in every JSON string fails to parse.

**Trigger + numbers.** `"A"`: C → `"A"`; Rust → parse error. `"𝄞"` (𝄞 surrogate
pair): C → `f0 9d 84 9e`; Rust → error. This class drives **26,657 of the 40,133 divergences**
(120,050 UB-free records, all in `parse_string`; ASan/UBSan 0 reports; controls `parse_hex4`,
`parse_number`, standalone `utf16_literal_to_utf8` all 0 diffs). **Why the matcher/differential is
needed:** the callee alone verifies clean — only differential *through the caller* finds it.

---

## S8 — cJSON × PtrTrans — `valuestring = None`: parse "succeeds", value discarded (headline #2, part 2/3)

**Class:** semantic (data loss with success return)
**Evidence:** same archive, `translated_crate/src/cjson.rs:854`

**Mechanism.** On `parse_string`'s success path, C stores the freshly allocated string into
`item->valuestring`. PtrTrans couldn't tie a `&mut str` from a local `Vec` to `item` without unsafe
plumbing — so, as its own comment admits, it assigned `None` and still returned success.

**C contract** (via oracle, which reads *and frees* `item.valuestring` — proving C populates it;
`oracle.c:24`):
```c
            cJSON_bool r = parse_string(&item,&buffer);
            printf("S %d off=%zu vs=", (int)r, buffer.offset);
            if (item.valuestring){ phex((unsigned char*)item.valuestring, strlen(item.valuestring));
                                   global_hooks.deallocate(item.valuestring);} else printf("-");
```

**Rust — PtrTrans, buggy** (`translated_crate/src/cjson.rs:854`):
```rust
    if let Ok(s) = core::str::from_utf8(&output[..str_len]) {
        // We cannot safely create &mut str tied to item from a local Vec without unsafe or
        // a custom allocator hook. The original C code stores the allocated pointer into item.
        // Here we set valuestring to None to respect Rust safety while preserving logic elsewhere.
        item.valuestring = None; // <-- BUG: parsed value thrown away; returns success (1) anyway
        let _ = s;
```

**The defect.** The success branch discards the parsed bytes and returns `ret=1, type=string,
valuestring=NULL`. Every downstream consumer silently sees NULL. The translation *documents its own
bug in a comment* — "to respect Rust safety" — and ships.

**Trigger + numbers.** `"plain"` → C `valuestring="plain"`, Rust `None` — both sides report success.
**9,802 both-succeed divergences.** Undetectable by any crash/error-code oracle; only value-level
differential sees it.

---

## S9 — cJSON × PtrTrans — non-UTF-8 strings rejected: the `&str` idiom narrows C's byte semantics (headline #2, part 3/3)

**Class:** semantic (input-domain narrowing)
**Evidence:** same archive, `translated_crate/src/cjson.rs:854,860`

**Mechanism.** C cJSON stores raw string bytes — no UTF-8 validation exists. The lift assembles the
parsed bytes and then wraps them in `core::str::from_utf8`; the `Err` branch fails the whole parse.
A validation step that C never had.

**Rust — PtrTrans, buggy** (`translated_crate/src/cjson.rs:854`):
```rust
    if let Ok(s) = core::str::from_utf8(&output[..str_len]) {   // <-- BUG: gate C never had
        ...
    } else {
        let ip_copy = input_pointer_index;
        let _ = fail_with_offset(&mut current_offset, Some(ip_copy));
        input_buffer.offset = current_offset;
        return 0;                                               // whole parse fails on high bytes
    }
```

**The defect.** Any JSON string containing non-UTF-8 bytes (C: accepted, stored raw) fails the whole
parse in Rust. Same *family* as the C2SaferRust UTF-8 panics (C2-C5) but a different manifestation:
there the program dies; here it politely returns an error the C API would never produce — quieter,
and equally a divergence.

**Trigger.** `"\xff\xfe raw high bytes"` → C `ret=1` (17 raw bytes stored); Rust `ret=0`. Counted
within S7's class total. Bonus surfaced by the same run: `"\uZZZZ"` → C `ret=1` yielding U+0000 (its
`parse_hex4` has no error channel!), Rust `ret=0` — a case where *C's* behavior is the questionable
one; the differential surfaces it either way.

---

## S10 ★ bzip2 × CROWN — compress returns BZ_OK, emits corrupt streams: 46% (headline #3)

**Class:** semantic (silent corruption with success return) · **ownership-lift breakage**
**Evidence:** `results/rq1_bugs/bzip2_crown/` — *behavioral attribution (see C7 note: no line-level
CROWN diff in the archive; base c2rust byte-exact ≡ C on the same corpus isolates the defect to
CROWN's rewrite).*

**Mechanism.** On the ~46% of inputs that don't crash (C7), CROWN's rewritten compressor returns
`BZ_OK` but emits a structurally invalid bzip2 stream — canonical `bunzip2` rejects it with "Data
integrity error."

**The reproducing call + archived artifact** (`crown_compress_driver.rs:16`):
```rust
            if let Ok(pth)=std::env::var("CROWN_OUT"){ if rc==0 {std::fs::write(pth,&out[..dl as usize]).unwrap();} }
            if rc!=0{println!("rc={}",rc);}
            else{println!("ok len={} fnv={:016x}",dl,fnv(&out[..dl as usize]));}
```

**The defect (behavioral).** Input `"A"*4096`, blockSize100k=1: C/base emit a 46-byte stream; CROWN
emits **54 bytes** that `bunzip2` rejects (0 bytes out). The corrupt output is archived
(`crown_corrupt_A4096.bz2`). `rc == 0` in every corrupt case — **zero error returns across the
corpus**.

**Numbers.** 150-record sample: **43 roundtrip-OK / 69 corrupt (46%) / 38 crash (25%)** — only 29%
correct. The irony headline: the mechanical baseline was safe and correct; the *safety* lift is what
broke it.

---

## S11 — bzip2 × CROWN — decompress fast path (`small=0`) rejects valid data (headline #3, sibling)

**Class:** semantic · **ownership-lift breakage**
**Evidence:** same archive, `crown_decompress_driver.rs`

**Mechanism.** `BZ2_bzBuffToBuffDecompress` with `small=0` (the default fast path) returns
`BZ_DATA_ERROR (-4)` on streams verified valid by canonical `bunzip2`. The `small=1` low-memory path
works. Base c2rust decompresses correctly on both — CROWN broke specifically the default state
machine.

**The reproducing driver** (`crown_decompress_driver.rs:6`):
```rust
    for small in [0i32,1] {
        let mut back=vec![0u8; 1<<20];
        let mut bl: u32 = back.len() as u32;
        let rd = unsafe { BZ2_bzBuffToBuffDecompress(back.as_mut_ptr() as *mut i8, Some(&mut bl),
            comp.as_ptr() as *mut i8, comp.len() as u32, small, 0) };
        println!("small={} rd={} bl={} out={:?}", small, rd, bl, ...);
    }
```

**Numbers.** `small=0`: `-4` on every valid stream tested; `small=1`: correct output. An API whose
*default* configuration cannot decompress.

---

## S12 — bzip2 × C2SaferRust — `BZ2_bzBuffToBuffCompress` rejects the valid empty buffer

**Class:** semantic · **NULL/empty conflation** (the same lift error as S1, at an API entry point)
**Evidence:** no dedicated archive dir; Rust from shipped `bzip2_WIP/bzlib.rs:2085`, C from `crown/c-code/bzip2/bzlib.c:1247`; documented in `results/rq1_semantic_diffs.md` row 3

**Mechanism.** The C API's param check rejects only NULL pointers; `sourceLen == 0` (a valid empty
buffer) compresses to a valid empty stream. The lift folded the NULL guards into `.is_empty()` tests.

**C** (`bzlib.c:1247`):
```c
int BZ_API(BZ2_bzBuffToBuffCompress)
                         ( char*         dest,
                           unsigned int* destLen,
                           char*         source,
                           unsigned int  sourceLen,
                           int           blockSize100k,
                           int           verbosity,
                           int           workFactor )
{
   ...
   if (dest == NULL || destLen == NULL ||
       source == NULL ||
       blockSize100k < 1 || blockSize100k > 9 ||
       verbosity < 0 || verbosity > 4 ||
       workFactor < 0 || workFactor > 250)
      return BZ_PARAM_ERROR;
```

**Rust — C2SaferRust WIP, buggy** (`bzip2_WIP/bzlib.rs:2085`):
```rust
pub fn BZ2_bzBuffToBuffCompress(dest: &mut Vec<u8>, source: &[u8], blockSize100k: i32, verbosity: i32, workFactor: i32) -> i32 {
    if dest.is_empty() || source.is_empty() || blockSize100k < 1 || blockSize100k > 9 || verbosity < 0 || verbosity > 4 || workFactor < 0 || workFactor > 250 {
        return -2;    // <-- BUG: `source == NULL` became `source.is_empty()`; valid empty input rejected
    }
```

**The defect.** `source == NULL` → `source.is_empty()`: compressing an empty buffer returns
`BZ_PARAM_ERROR (-2)` where C returns `BZ_OK` plus a valid empty `.bz2` stream. (Note `dest` — an
output buffer whose *capacity* matters — gets the same wrong treatment.) Second member of the
conflation class after S1/S2: same lift error at a checksum boundary and an API boundary.

**Trigger.** Compress `len=0` → C: `BZ_OK` + valid stream; WIP: `-2`.

---

## S13 — tulipindicators × C2SaferRust — argc off-by-one in the rewritten `main()` wrapper

**Class:** semantic (valid invocations rejected) · **driver/guard rewrite**
**Evidence:** `results/rq1_bugs/tulip_c2saferrust/` (Bug 2, `excerpt_argc_offbyone.rs`)

**Mechanism.** c2rust's mechanical `main()` wrapper builds `[prog, args..., NULL]` and passes the
true argc. C2SaferRust's rewrite counts `env::args()` — which already excludes the NULL it pushes
later — and *still* subtracts one: every invocation reaches `main_0` one argument poorer.

**Rust — C2SaferRust WIP, buggy** (`tulipindicators_WIP/sample.rs:362`):
```rust
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = (args.len() - 1) as std::os::raw::c_int;   // <-- BUG: env::args() has no NULL to subtract
    let arg_ptrs: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    ...
    raw_args.push(std::ptr::null_mut());
    unsafe {
        let exit_code = main_0(arg_count, raw_args.as_mut_ptr());
```
The consuming check that then misfires (`sample.rs:245`):
```rust
for i_0 in 0..info.as_ref().unwrap().options {
    if argc < 3 + i_0 {
        eprintln!("*ERROR NOT ENOUGH OPTIONS*");
        return 1;
    }
```

**The defect.** c2rust's original accounting subtracted 1 for the trailing NULL it appended to the
pointer array; the rewrite kept the `- 1` but built the count from `env::args().len()`, which never
included a NULL. Result: `argc` is one less than reality on every run.

**Trigger + numbers.** `sample sma 5` (valid) → C/base prints the SMA table (exit 0); WIP prints
`*ERROR NOT ENOUGH OPTIONS*` (exit 1). Over-supplying (`sample sma 5 5`) "fixes" it. No crash, both
terminate cleanly — **invisible to fuzz-Rust-alone**, found only because the differential compared
driver behavior. Value layer meanwhile faithful: 150k records × 11 indicators byte-identical — both
tulip bugs live in the translated driver/wrapper, not indicator math (scope stated honestly).

---
---

# Cross-cutting summary (for the meeting)

1. **Zeroed-table corruption is the dominant class — 4 instances, 3 tools, 3 distinct mechanisms:**
   uncalled init (S3, S4 — Laertes), NULL/empty conflation resetting a running checksum (S1 —
   C2SaferRust), mutability loss erasing the init's writes (S5 — SACTOR). One symptom: a table that
   silently stays zero. `crc32_z` alone is broken by two tools in two different ways (S1 vs S4).
2. **The silent-semantic core:** S1, S3, S4, S5, S6, S7, S8, S10 all return success / never panic —
   no Rust-side fuzzer or crash oracle can see them; only C-backed differential does. S8 even
   *documents its own data loss in a comment* and ships.
3. **Safety idioms are the bug vector, repeatedly:** `is_empty()` for `is_null()` (S1 S2 S12),
   `to_str().unwrap()` (C2-C5), `Option`-swap swallowing errors (S6), `from_utf8` gate (S9),
   immutable static (S5). The idiom is what makes the bug *quiet*.
4. **Per-function verification ≢ whole-program correctness:** SACTOR passed 15/15 of its own
   per-function FFI tests (S5); PtrTrans's callee verifies clean standalone while the call site is
   broken (S7); PtrTrans's cargo gate passed the sort that doesn't sort (S6).
5. **The irony:** the one memory-corruption bug in the ledger (C7) was introduced by CROWN — the
   *safety* lifter — on code the mechanical baseline translated safely.

### Archive-honesty appendix (what is and isn't pinned to code)

- **bzip2 × CROWN (C7, S10, S11):** behavioral attribution only — the archive has drivers, oracle,
  corpus numbers, and one archived corrupt output, but no line-level CROWN-vs-base source diff (the
  code-level scratch lived on wiped /tmp). Do not cite specific CROWN lines as "the" defect.
- **cJSON originals:** the modern cJSON.c the oracle `#include`s is not in the repo; the C contract
  is evidenced through `oracle.c` (which passes real bounds / reads `valuestring`) — quoted above.
- **Checksum C sides (S1-S4):** original zlib/bzip2 `.c` not archived; the faithful base-c2rust Rust
  (verified ≡ C by the differential) is quoted as the reference. bzlib.c and bzip2recover.c C
  originals ARE in-repo (`crown/c-code/bzip2/`) and quoted directly for C4/S12.
- **adler32 (S2):** messier than S1 (tool's own log says Failure; extra miscompile on non-empty
  input) — corroborates the class; keep S1 as the clean exhibit.
- **tulip C source:** not in repo; base c2rust is the reference (S13's contract is additionally
  evidenced by the WIP's own `argc < 3 + i_0` check).
