#include <stdio.h>
#include <string.h>
#include "url.h"
extern volatile int c2r_ub_flag;
void c2r_ub_reset(void); int c2r_ub_get(void);
int main(void){
  static char buf[1<<16]; size_t n=fread(buf,1,sizeof buf-1,stdin); buf[n]=0;
  c2r_ub_reset();
  url_data_t* d=url_parse(buf);
  (void)url_get_protocol(buf);(void)url_get_host(buf);(void)url_get_path(buf);
  fprintf(stderr,"UB_FLAG_AFTER_PARSE=%d\n", c2r_ub_get());  /* does the UBSan gate see the heap overflow? */
  if(d) url_free(d);
  return 0;
}
