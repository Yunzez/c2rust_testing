#include "bzlib.h"
#include <stdio.h>
#include <string.h>
#include <stdint.h>
static uint64_t fnv(const unsigned char*p,size_t n){uint64_t h=1469598103934665603ULL;for(size_t i=0;i<n;i++){h^=p[i];h*=1099511628211ULL;}return h;}
int main(void){
    static unsigned char in[1<<20],out[(1<<20)+1200000];
    unsigned char hdr[4],par[2];
    while(fread(hdr,1,4,stdin)==4){
        unsigned len=hdr[0]|(hdr[1]<<8)|(hdr[2]<<16)|(hdr[3]<<24);
        if(len>sizeof(in))break;
        if(len&&fread(in,1,len,stdin)!=len)break;
        if(fread(par,1,2,stdin)!=2)break;
        unsigned dl=sizeof(out);
        int rc=BZ2_bzBuffToBuffCompress((char*)out,&dl,(char*)in,len,par[0],0,par[1]);
        if(rc!=BZ_OK){printf("rc=%d\n",rc);continue;}
        printf("ok len=%u fnv=%016llx\n",dl,(unsigned long long)fnv(out,dl));
    }
    return 0;
}
