struct _IO_FILE;
struct _IO_FILE;
struct _IO_FILE;
struct _IO_marker;
struct _IO_codecvt;
struct _IO_wide_data;
int u8next_(const char *txt, int *ch);
int u8next_(const char *txt, int *ch)
{
  int len = 0;
  unsigned char *s = (unsigned char *)txt;
  char first = *s;
  int val = 0;
  if (first) {
    val = first;
    goto fsm_state_START; {
      fsm_state_START : {
        if (*s < 0x80) { len = 1; goto fsm_state_end; }
        if (*s == 0xC0) { len = 2; goto fsm_state_null; }
        if (*s <= 0xC1) { goto fsm_state_invalid; }
        if (*s <= 0xDF) { val &= 0x1F; len = 2; goto fsm_state_len2_0; }
        if (*s == 0xE0) { val &= 0x0F; len = 3; goto fsm_state_len3_0; }
        if (*s <= 0xEC) { val &= 0x0F; len = 3; goto fsm_state_len3_1; }
        if (*s == 0xED) { val &= 0x0F; len = 3; goto fsm_state_len3_2; }
        if (*s <= 0xEF) { val &= 0x0F; len = 3; goto fsm_state_len3_1; }
        if (*s == 0xF0) { val &= 0x07; len = 4; goto fsm_state_len4_0; }
        if (*s <= 0xF3) { val &= 0x07; len = 4; goto fsm_state_len4_1; }
        if (*s == 0xF4) { val &= 0x07; len = 4; goto fsm_state_len4_2; }
        goto fsm_state_invalid;
      }
      fsm_state_null : {
        val = 0;
        s++; if ( *s != 0x80) goto fsm_state_invalid;
        goto fsm_state_end;
      }
      fsm_state_len4_0 : {
        s++; if ( *s < 0x90 || 0xBF < *s) goto fsm_state_invalid;
        goto fsm_state_len4;
      }
      fsm_state_len4_1 : {
        s++; if ( *s < 0x80 || 0xBF < *s) goto fsm_state_invalid;
        goto fsm_state_len4;
      }
      fsm_state_len4_2 : {
        s++; if ( *s < 0x80 || 0x8F < *s) goto fsm_state_invalid;
        goto fsm_state_len4;
      }
      fsm_state_len4 : {
        val = (val << 6) | (*s & 0x3F);
        goto fsm_state_len3_1;
      }
      fsm_state_len3_0 : {
        s++; if ( *s < 0xA0 || 0xBF < *s) goto fsm_state_invalid;
        goto fsm_state_len3;
      }
      fsm_state_len3_1 : {
        s++; if ( *s < 0x80 || 0xBF < *s) goto fsm_state_invalid;
        goto fsm_state_len3;
      }
      fsm_state_len3_2 : {
        s++; if ( *s < 0x80 || 0x9F < *s) goto fsm_state_invalid;
        goto fsm_state_len3;
      }
      fsm_state_len3 : {
        val = (val << 6) | (*s & 0x3F);
        goto fsm_state_len2_0;
      }
      fsm_state_len2_0 : {
        s++; if ( *s < 0x80 || 0xBF < *s) goto fsm_state_invalid;
        val = (val << 6) | (*s & 0x3F);
        goto fsm_state_end;
      }
      fsm_state_invalid : {val = first; len = -1;}
      fsm_state_end : { }
    }
  }
  if (ch) *ch = val;
  return len;
}

int c_u8next(const char* t, int* ch){ return u8next_(t, ch); }
