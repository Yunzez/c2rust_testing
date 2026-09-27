# C-guided companion campaign — lil_c2rust

Same harnesses (generator `fd1f75b4716a1d45`, `--c-coverage`), same seed and libFuzzer parameters as the archived
campaign, `C2R_MODE=c-only`, budget 3600 s, 50 boundaries. CR = the archived Rust-guided corpus,
CC = this C-guided corpus. Reach only; no candidate from CC is adjudicated here.

## 2 x 2 (corpus x side); percentages are side-specific and never subtracted

| corpus | C functions | C regions | Rust functions | Rust regions |
|---|---|---|---|---|
| Rust-guided CR | 139 / 145 | 1887 / 2249 (0.839) | 143 / 151 | 4999 / 5730 (0.872) |
| C-guided CC | 140 / 145 | 1883 / 2249 (0.837) | 144 / 151 | 4868 / 5730 (0.850) |
| CR ∪ CC | 140 / 145 | 1942 / 2249 (0.863) | 144 / 150 | – |

C-side inputs on CC: {'completed': 3554, 'crash': 7}; corpus sizes 3561 inputs.

## Matched-function sets (accepted pairs ∩ C scope ∩ Rust scope)

| corpus | pairs | both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous |
|---|---|---|---|---|---|---|
| CR | 127 | 123 | 0 (0) | 0 (0) | 4 | 18 |
| CC | 127 | 124 | 0 (0) | 0 (0) | 3 | 18 |
| CR ∪ CC | 127 | 124 | 0 (0) | 0 (0) | 3 | 18 |

## Campaign

| boundary | corpus | jobs | cov | crash | timeout |
|---|---|---|---|---|---|
| add_func | 30 | 5623659 | 264 | 0 | 0 |
| alloc_value | 14 | 97643898 | 136 | 0 | 0 |
| alloc_value_len | 14 | 91872633 | 136 | 14 | 0 |
| ateol | 1 | 42029 | 0 | 42028 | 0 |
| ee_invalidpunct | 6 | 206075 | 111 | 43230 | 0 |
| find_cmd | 32 | 5959656 | 264 | 0 | 0 |
| fnc_embed_write | 14 | 6207279 | 267 | 0 | 0 |
| get_bracketpart | 1 | 5986232 | 232 | 0 | 0 |
| get_dollarpart | 1 | 42173 | 0 | 42172 | 0 |
| hm_hash | 23 | 106306818 | 135 | 0 | 0 |
| islilspecial | 9 | 278107942 | 110 | 0 | 0 |
| lil_alloc_double | 10 | 831626 | 124 | 37337 | 0 |
| lil_alloc_integer | 8 | 189085060 | 124 | 7 | 0 |
| lil_alloc_list | 1 | 224797453 | 96 | 5 | 0 |
| lil_alloc_string | 14 | 101591428 | 136 | 0 | 0 |
| lil_alloc_string_len | 14 | 94072606 | 137 | 8 | 0 |
| lil_append_char | 10 | 156875269 | 135 | 0 | 0 |
| lil_append_string | 10 | 8115727 | 136 | 37265 | 0 |
| lil_append_string_len | 19 | 10173824 | 161 | 34536 | 0 |
| lil_clone_value | 8 | 143365895 | 128 | 5 | 0 |
| lil_embedded | 1345 | 3230352 | 1899 | 3 | 1 |
| lil_free | 1 | 5892667 | 223 | 0 | 0 |
| lil_free_list | 1 | 211970162 | 98 | 0 | 0 |
| lil_free_value | 8 | 168618151 | 126 | 0 | 0 |
| lil_freemem | 1 | 239701844 | 95 | 0 | 0 |
| lil_get_data | 1 | 6328503 | 225 | 0 | 0 |
| lil_get_var | 30 | 5606161 | 272 | 0 | 0 |
| lil_list_get | 8 | 213816861 | 127 | 0 | 0 |
| lil_list_to_value | 5 | 188615201 | 117 | 1 | 0 |
| lil_new | 1 | 4196197 | 195 | 72 | 0 |
| lil_parse | 1690 | 3202272 | 1882 | 5 | 7 |
| lil_pop_env | 1 | 6158373 | 227 | 0 | 0 |
| lil_push_env | 1 | 5635644 | 228 | 0 | 0 |
| lil_set_data | 1 | 6185928 | 225 | 0 | 0 |
| lil_set_error | 14 | 5727845 | 267 | 0 | 0 |
| lil_set_error_at | 22 | 5825535 | 290 | 0 | 0 |
| lil_to_boolean | 12 | 154689725 | 139 | 0 | 0 |
| lil_to_double | 8 | 162478504 | 128 | 0 | 0 |
| lil_to_integer | 8 | 156310781 | 128 | 0 | 0 |
| lil_to_string | 8 | 159989596 | 128 | 0 | 0 |
| lil_unused_name | 27 | 5743230 | 271 | 0 | 0 |
| lil_write | 16 | 5796681 | 264 | 0 | 0 |
| needs_escape | 22 | 98297833 | 145 | 0 | 0 |
| next_word | 1 | 42029 | 0 | 42028 | 0 |
| real_inc | 7 | 2807164 | 265 | 19791 | 0 |
| real_trim | 65 | 90281822 | 202 | 2 | 0 |
| register_stdcmds | 1 | 5766293 | 228 | 0 | 0 |
| skip_spaces | 1 | 6063658 | 227 | 0 | 0 |
| strclone | 15 | 104543600 | 133 | 0 | 0 |
| substitute | 1 | 6199637 | 232 | 0 | 0 |

## Procedure, deviations, and what is not established

<!-- prose -->
