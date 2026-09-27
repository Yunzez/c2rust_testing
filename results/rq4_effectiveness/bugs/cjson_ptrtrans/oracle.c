/* C oracle for per-function differential vs PtrTrans cJSON translation.
   stdin: records [op u8][len u16 LE][payload]. One canonical line per record. */
#include "cJSON.c"   /* expose statics */
#include <stdio.h>
#include <string.h>
static void phex(const unsigned char*p, size_t n){ for(size_t i=0;i<n;i++) printf("%02x",p[i]); }
int main(void){
    unsigned char op; unsigned char hdr[2]; static unsigned char pay[1<<16];
    while (fread(&op,1,1,stdin)==1) {
        if (fread(hdr,1,2,stdin)!=2) break;
        size_t len = hdr[0] | (hdr[1]<<8);
        if (len && fread(pay,1,len,stdin)!=len) break;
        parse_buffer buffer; memset(&buffer,0,sizeof buffer);
        buffer.content = pay; buffer.length = len; buffer.offset = 0; buffer.hooks = global_hooks;
        cJSON item; memset(&item,0,sizeof item);
        if (op==0) { /* parse_hex4: needs >=4 bytes (C reads blind) */
            if (len<4) { printf("H skip\n"); continue; }
            printf("H %08x\n", parse_hex4(pay));
        } else if (op==1) { /* parse_number */
            cJSON_bool r = parse_number(&item,&buffer);
            unsigned long long bits; memcpy(&bits,&item.valuedouble,8);
            printf("N %d off=%zu int=%d dbl=%016llx ty=%d\n", (int)r, buffer.offset, item.valueint, bits, item.type);
        } else if (op==2) { /* parse_string */
            cJSON_bool r = parse_string(&item,&buffer);
            printf("S %d off=%zu vs=", (int)r, buffer.offset);
            if (item.valuestring){ phex((unsigned char*)item.valuestring, strlen(item.valuestring)); global_hooks.deallocate(item.valuestring);} else printf("-");
            printf(" ty=%d\n", item.type);
        } else if (op==3) { /* utf16_literal_to_utf8 */
            unsigned char out[64]; unsigned char*po=out;
            unsigned char r = utf16_literal_to_utf8(pay, pay+len, &po);
            printf("U %u out=", (unsigned)r); phex(out,(size_t)(po-out)); printf("\n");
        }
    }
    return 0;
}
