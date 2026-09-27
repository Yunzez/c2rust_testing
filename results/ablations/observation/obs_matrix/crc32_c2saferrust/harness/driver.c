/* OBS matrix driver (C side): incremental CRC-32 (zlib crc32_z as vendored by optipng).
   argv[1] = silent|print ; argv[2] = state file ; stdin = input bytes.
   Decoding (identical in Rust driver + libFuzzer target): bytes[0..4] = seed crc (u32 LE);
   then a chunk stream: 1 byte L, followed by min(L, remaining) bytes -> crc = crc32_z(crc, chunk, chunklen).
   A zero-length chunk (L==0) is a legal empty write segment (the optipng IDAT accumulation shape). */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "zlib.h"
int main(int argc,char**argv){
    if(argc<3) return 2;
    int printing=strcmp(argv[1],"print")==0;
    static unsigned char buf[1<<16]; size_t len=0,r;
    while((r=fread(buf+len,1,sizeof(buf)-len,stdin))>0) len+=r;
    unsigned long crc=0; size_t p=0; unsigned nchunks=0;
    if(len>=4){ crc=(unsigned long)(buf[0]|(buf[1]<<8)|(buf[2]<<16)|((unsigned)buf[3]<<24)); p=4; }
    while(p<len){ size_t L=buf[p++]; if(L>len-p) L=len-p; crc=crc32_z(crc,buf+p,L); p+=L; nchunks++; }
    FILE*sf=fopen(argv[2],"w"); if(!sf) return 3;
    fprintf(sf,"ret:0x%08lx\nchunks:%u\nglobals:none\n",crc,nchunks); fclose(sf);
    if(printing) printf("crc=0x%08lx\n",crc);
    return 0;
}
