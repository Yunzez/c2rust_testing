/* OOP oracle for rgba_from_string(const char* str, short* ok) -> uint32_t.
 * stdin bytes -> NUL-terminated string -> call C -> serialize (ret + ok). ASan/UBSan gate. */
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include "rgba.h"

static unsigned char _buf[1<<16]; static size_t _got, _pos;
static unsigned char next_byte(void){ return _pos < _got ? _buf[_pos++] : (_pos++, 0); }

int main(void){
    _got = fread(_buf,1,sizeof _buf,stdin); _pos=0;
    size_t n = (size_t)(next_byte() % 65);
    char* str = (char*)malloc(n+1);
    for (size_t i=0;i<n;i++) str[i]=(char)next_byte();
    str[n]=0;
    short ok = 0;
    /* isolate the callee's stdout (rgba doesn't print, but keep the discipline) */
    fflush(stdout); int _so=dup(1); int _dn=open("/dev/null",O_WRONLY); if(_dn>=0) dup2(_dn,1);
    uint32_t ret = rgba_from_string(str, &ok);
    fflush(stdout); if(_dn>=0){ dup2(_so,1); close(_dn);} if(_so>=0) close(_so);
    printf("ret:%u ok:%d\n", ret, (int)ok);
    free(str);
    return 0;
}
