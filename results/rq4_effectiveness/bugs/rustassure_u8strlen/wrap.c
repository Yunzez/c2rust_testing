int u8strlen(const char *s){ int len=0; while(*s){ if((*s & 0xC0)!=0x80) len++; s++; } return len; }
int c_u8strlen(const char* s){ return u8strlen(s); }
