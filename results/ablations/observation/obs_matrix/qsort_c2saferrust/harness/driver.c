/* OBS pilot driver (C side). argv[1] = "silent" | "print"; argv[2] = state-file path.
   stdin = raw bytes -> i32 LE chunks (take 256), identical decoding to the libFuzzer target.
   silent: prints NOTHING to stdout, exit 0.   print: prints the sorted array to stdout.
   Both write the boundary state (return + array + globals) to the state file (NOT stdout). */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "qsort.c"
int main(int argc, char **argv){
    if(argc<3) return 2;
    int printing = strcmp(argv[1],"print")==0;
    unsigned char buf[4096]; size_t len=0, r;
    while((r=fread(buf+len,1,sizeof(buf)-len,stdin))>0) len+=r;
    int n=(int)(len/4); if(n>256) n=256;
    int *a = malloc(sizeof(int)*(n>0?n:1));
    for(int i=0;i<n;i++){ unsigned v=(unsigned)buf[4*i]|((unsigned)buf[4*i+1]<<8)|((unsigned)buf[4*i+2]<<16)|((unsigned)buf[4*i+3]<<24); memcpy(&a[i],&v,4); }
    quickSort(a,0,n-1);
    FILE *sf=fopen(argv[2],"w"); if(!sf) return 3;
    fprintf(sf,"ret:void\nglobals:none\narr:");
    for(int i=0;i<n;i++) fprintf(sf,"%d ",a[i]);
    fprintf(sf,"\n"); fclose(sf);
    if(printing){ for(int i=0;i<n;i++) printf("%d ",a[i]); printf("\n"); }
    free(a);
    return 0;
}
