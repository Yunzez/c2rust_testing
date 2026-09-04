#include "url.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void){
  static char buf[1<<16]; size_t n=fread(buf,1,sizeof buf-1,stdin); buf[n]=0;
  url_data_t* d=url_parse(buf);
  char* a=url_get_protocol(buf); char* b=url_get_host(buf); char* c=url_get_path(buf);
  printf("proto=%s\nhost=%s\npath=%s\n", a?a:"(null)", b?b:"(null)", c?c:"(null)");
  if(d) url_free(d);
  return 0;
}
