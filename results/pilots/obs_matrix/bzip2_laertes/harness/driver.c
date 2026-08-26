/* OBS driver (C side, bzip2 C source under ASan+UBSan). argv[1]=silent|print, argv[2]=state file.
   stdin bytes -> src buffer (cap 64 KiB); BZ2_bzBuffToBuffCompress(dest,&dl,src,len,1,0,30) once.
   State file (written AFTER the call, never to stdout): ret:<rc> / destLen:<n> / globals:none / out:<hex>.
   print mode: 'rc=<rc> len=<n>' + hex of output to stdout; silent prints nothing. */
#include "bzlib.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main(int argc,char**argv){
    if(argc<3) return 2;
    int printing=strcmp(argv[1],"print")==0;
    static unsigned char in[65536]; size_t len=0,r;
    while(len<sizeof(in)&&(r=fread(in+len,1,sizeof(in)-len,stdin))>0) len+=r;
    unsigned dl=65536*2+1200; unsigned char*out=malloc(dl);
    int rc=BZ2_bzBuffToBuffCompress((char*)out,&dl,(char*)in,(unsigned)len,1,0,30);
    if(rc!=BZ_OK) dl=0;
    FILE*sf=fopen(argv[2],"w"); if(!sf) return 3;
    fprintf(sf,"ret:%d\ndestLen:%u\nglobals:none\nout:",rc,dl);
    for(unsigned i=0;i<dl;i++) fprintf(sf,"%02x",out[i]);
    fprintf(sf,"\n"); fclose(sf);
    if(printing){ printf("rc=%d len=%u\n",rc,dl); for(unsigned i=0;i<dl;i++) printf("%02x",out[i]); printf("\n"); }
    free(out); return 0;
}
