#include <zlib.h>
#include <stdio.h>
#include <stdlib.h>
int main(void){
    unsigned char hdr[2]; static unsigned char buf[70000];
    while(fread(hdr,1,2,stdin)==2){
        unsigned len=hdr[0]|(hdr[1]<<8);
        if(len>sizeof(buf))break;
        if(len&&fread(buf,1,len,stdin)!=len)break;
        unsigned long c=crc32(0L,buf,len);
        unsigned long a=adler32(1L,buf,len);
        printf("crc=%08lx adler=%08lx\n",c&0xffffffff,a&0xffffffff);
    }
    return 0;
}
