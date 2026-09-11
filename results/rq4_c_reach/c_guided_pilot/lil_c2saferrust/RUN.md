# C-guided companion campaign — lil_c2saferrust

Same harnesses (generator `fd1f75b4716a1d45`, `--c-coverage`), same seed and libFuzzer parameters as the archived
campaign, `C2R_MODE=c-only`, budget 3600 s, 47 boundaries. CR = the archived Rust-guided corpus,
CC = this C-guided corpus. Reach only; no candidate from CC is adjudicated here.

## 2 x 2 (corpus x side); percentages are side-specific and never subtracted

| corpus | C functions | C regions | Rust functions | Rust regions |
|---|---|---|---|---|
| Rust-guided CR | 62 / 145 | 457 / 2249 (0.203) | 25 / 154 | 362 / 5751 (0.063) |
| C-guided CC | 138 / 145 | 1846 / 2249 (0.821) | 25 / 154 | 363 / 5751 (0.063) |
| CR ∪ CC | 138 / 145 | 1847 / 2249 (0.821) | 24 / 150 | – |

C-side inputs on CC: {'completed': 3568, 'crash': 7}; corpus sizes 3575 inputs.

## Matched-function sets (accepted pairs ∩ C scope ∩ Rust scope)

| corpus | pairs | both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous |
|---|---|---|---|---|---|---|
| CR | 119 | 24 | 37 (37) | 0 (0) | 58 | 25 |
| CC | 119 | 24 | 92 (92) | 0 (0) | 3 | 25 |
| CR ∪ CC | 119 | 24 | 92 (0) | 0 (0) | 3 | 25 |

`c_only` on CC: hm_hash, hm_init, hm_destroy, hm_put, hm_get, lil_append_val, lil_list_append, lil_alloc_env, lil_free_env, lil_find_local_var, lil_find_var, find_cmd, add_func, del_func, lil_register, lil_set_var, lil_get_var, lil_get_var_or, lil_push_env, lil_pop_env, lil_new, ateol, skip_spaces, get_bracketpart, get_dollarpart, next_word, substitute, lil_subst_to_list, lil_subst_to_value, lil_parse, lil_parse_value, lil_set_error, lil_set_error_at, ee_unary, ee_muldiv, ee_addsub, ee_shift, ee_compare, ee_equals, ee_bitand, ee_bitor, lil_eval_expr, lil_unused_name, lil_free, lil_set_data, lil_get_data, lil_embedded, lil_write, fnc_reflect, fnc_func, fnc_rename, fnc_unusedname, fnc_quote, fnc_set, fnc_write, fnc_print, fnc_eval, fnc_upeval, fnc_enveval, fnc_jaileval …

`c_only` on CR ∪ CC: hm_hash, hm_init, hm_destroy, hm_put, hm_get, lil_append_val, lil_list_append, lil_alloc_env, lil_free_env, lil_find_local_var, lil_find_var, find_cmd, add_func, del_func, lil_register, lil_set_var, lil_get_var, lil_get_var_or, lil_push_env, lil_pop_env, lil_new, ateol, skip_spaces, get_bracketpart, get_dollarpart, next_word, substitute, lil_subst_to_list, lil_subst_to_value, lil_parse, lil_parse_value, lil_set_error, lil_set_error_at, ee_unary, ee_muldiv, ee_addsub, ee_shift, ee_compare, ee_equals, ee_bitand, ee_bitor, lil_eval_expr, lil_unused_name, lil_free, lil_set_data, lil_get_data, lil_embedded, lil_write, fnc_reflect, fnc_func, fnc_rename, fnc_unusedname, fnc_quote, fnc_set, fnc_write, fnc_print, fnc_eval, fnc_upeval, fnc_enveval, fnc_jaileval …

## Campaign

| boundary | corpus | jobs | cov | crash | timeout |
|---|---|---|---|---|---|
| add_func | 30 | 7713169 | 264 | 0 | 0 |
| alloc_value | 16 | 131598911 | 136 | 0 | 0 |
| alloc_value_len | 14 | 110616300 | 136 | 14 | 0 |
| ateol | 1 | 49198 | 0 | 49197 | 0 |
| ee_invalidpunct | 6 | 239649 | 111 | 50215 | 0 |
| find_cmd | 31 | 7819609 | 264 | 0 | 0 |
| fnc_embed_write | 15 | 7716107 | 267 | 0 | 0 |
| get_bracketpart | 1 | 7935148 | 232 | 0 | 0 |
| get_dollarpart | 1 | 49034 | 0 | 49033 | 0 |
| islilspecial | 9 | 377374957 | 110 | 0 | 0 |
| lil_alloc_double | 10 | 737274 | 124 | 43442 | 0 |
| lil_alloc_integer | 8 | 268288338 | 124 | 6 | 0 |
| lil_alloc_list | 1 | 325376506 | 96 | 7 | 0 |
| lil_alloc_string | 16 | 129497035 | 136 | 0 | 0 |
| lil_append_char | 10 | 206367057 | 135 | 0 | 0 |
| lil_append_string | 10 | 9434560 | 136 | 43377 | 0 |
| lil_append_string_len | 19 | 11860988 | 161 | 40309 | 0 |
| lil_clone_value | 8 | 201182703 | 128 | 7 | 0 |
| lil_embedded | 1604 | 4028207 | 2016 | 67 | 8 |
| lil_free | 1 | 8077098 | 223 | 0 | 0 |
| lil_free_list | 1 | 298343327 | 98 | 0 | 0 |
| lil_free_value | 8 | 235226354 | 126 | 0 | 0 |
| lil_get_data | 1 | 8196968 | 225 | 0 | 0 |
| lil_get_var | 28 | 7869601 | 272 | 0 | 0 |
| lil_list_get | 8 | 278393722 | 127 | 0 | 0 |
| lil_list_size | 1 | 291097844 | 100 | 0 | 0 |
| lil_list_to_value | 5 | 265328907 | 117 | 1 | 0 |
| lil_new | 1 | 5959988 | 195 | 60 | 0 |
| lil_parse | 1514 | 2732232 | 1853 | 106 | 24 |
| lil_pop_env | 1 | 7916702 | 227 | 0 | 0 |
| lil_push_env | 1 | 7342733 | 228 | 0 | 0 |
| lil_set_data | 1 | 8149174 | 225 | 0 | 0 |
| lil_set_error | 14 | 7896408 | 267 | 0 | 0 |
| lil_set_error_at | 20 | 7897828 | 290 | 0 | 0 |
| lil_to_boolean | 12 | 215166905 | 139 | 0 | 0 |
| lil_to_double | 8 | 215679946 | 128 | 0 | 0 |
| lil_to_integer | 8 | 226553226 | 128 | 0 | 0 |
| lil_to_string | 8 | 225564301 | 128 | 0 | 0 |
| lil_write | 14 | 7823207 | 264 | 0 | 0 |
| needs_escape | 22 | 133087852 | 145 | 0 | 0 |
| next_word | 1 | 49159 | 0 | 49158 | 0 |
| real_inc | 7 | 3263176 | 265 | 23100 | 0 |
| real_trim | 61 | 116642297 | 202 | 1 | 0 |
| register_stdcmds | 1 | 7396322 | 228 | 0 | 0 |
| skip_spaces | 1 | 8104532 | 227 | 0 | 0 |
| strclone | 16 | 133412092 | 133 | 0 | 0 |
| substitute | 1 | 8033289 | 232 | 0 | 0 |

## Procedure, deviations, and what is not established

<!-- prose -->
