# Second target (ACTIVE): safety-lifter validation

Apply the boundary-validity + UB-free differential-testing method to a SECOND translation tool — a
Rust→safer-Rust **safety lifter** that consumes c2rust output. This is "beyond c2rust" without needing
the full phase-2 semantic mapping, because lifting mostly preserves function names/structure.

Why a lifter (not a second C→Rust transpiler): a 2026-06-25 survey found c2rust is the ONLY
production-grade static C→Rust transpiler (Corrode/Citrus/CRUST are dead/prototype). Lifters are a
different tool doing a different translation (unsafe Rust → safe Rust), where bugs are PLAUSIBLE
(lifting changes pointers/aliasing/ownership; cf. OOPSLA'23 "Aliasing Limits", "superficial cleanup"
warnings) — unlike c2rust which is faithful-but-unsafe (we found 0 real bugs on it, as expected).

Candidates (prefer the STATIC ones — they preserve names → easy mapping):
- **Laertes** (Emre et al., OOPSLA'21) — static, compiler-feedback-driven raw-pointer→reference lifting.
- **CROWN** (CAV'23) — static, ownership-based pointer lifting.
- C2SaferRust (2025) — hybrid (c2rust + LLM); more renaming → harder mapping; lower priority.

Cleanest oracle: **lifted-Rust vs c2rust-Rust on the same inputs** (Rust↔Rust, names ~aligned, any
UB-free divergence is attributable to the LIFTER). Same boundary selection + UB-free exclusion (rule 4).

STATUS: runnability check first — do Laertes/CROWN actually build & run on our corpus in 2026?
(Research artifacts; runnability unverified.) See [[session-handoff]].
