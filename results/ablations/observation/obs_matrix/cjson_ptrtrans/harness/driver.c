/* OBS driver (C side). argv[1]=silent|print, argv[2]=state file. stdin = raw parse_buffer content (offset 0). */
#include "cJSON.c"   /* expose statics: parse_string, parse_buffer, global_hooks */
#include <stdio.h>
#include <string.h>
static void phex(FILE*f,const unsigned char*p,size_t n){ for(size_t i=0;i<n;i++) fprintf(f,"%02x",p[i]); }
int main(int argc,char**argv){
    if(argc<3) return 2;
    int printing = strcmp(argv[1],"print")==0;
    static unsigned char pay[1<<16]; size_t len=0,r;
    while((r=fread(pay+len,1,sizeof(pay)-len,stdin))>0) len+=r;
    parse_buffer buffer; memset(&buffer,0,sizeof buffer);
    buffer.content=pay; buffer.length=len; buffer.offset=0; buffer.hooks=global_hooks;
    cJSON item; memset(&item,0,sizeof item);
    cJSON_bool rc = parse_string(&item,&buffer);
    FILE*sf=fopen(argv[2],"w"); if(!sf) return 3;
    fprintf(sf,"ret:%d\ntype:%d\nvaluestring:",(int)rc,item.type);
    if(item.valuestring) phex(sf,(unsigned char*)item.valuestring,strlen(item.valuestring)); else fprintf(sf,"NULL");
    fprintf(sf,"\noffset:%zu\nglobals:none\n",buffer.offset); fclose(sf);
    if(printing){ printf("ret=%d valuestring=",(int)rc); if(item.valuestring) phex(stdout,(unsigned char*)item.valuestring,strlen(item.valuestring)); else printf("NULL"); printf("\n"); }
    if(item.valuestring) global_hooks.deallocate(item.valuestring);
    return 0;
}
