volatile int c2r_ub_flag = 0;
void c2r_ub_reset(void) { c2r_ub_flag = 0; }
int  c2r_ub_get(void)   { return c2r_ub_flag; }
#define H(name) void __ubsan_handle_##name##_minimal(void) { c2r_ub_flag = 1; }
H(add_overflow) H(sub_overflow) H(mul_overflow) H(negate_overflow)
H(divrem_overflow) H(shift_out_of_bounds) H(out_of_bounds)
H(type_mismatch) H(builtin_unreachable) H(pointer_overflow)
H(load_invalid_value)
#undef H
