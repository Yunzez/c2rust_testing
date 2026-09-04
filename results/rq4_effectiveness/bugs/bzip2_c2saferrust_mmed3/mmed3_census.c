#include <stdio.h>
typedef unsigned char UChar;
static UChar c_mmed3(UChar a, UChar b, UChar c){ UChar t;
  if (a>b){t=a;a=b;b=t;} if (b>c){ b=c; if (a>b) b=a; } return b; }
static UChar rs_mmed3(UChar a, UChar b, UChar c){ UChar m=a; if(b<m)m=b; if(c<m)m=c; return m; }
int main(void){ long tot=0,diff=0;
  for(int a=0;a<256;a++)for(int b=0;b<256;b++)for(int c=0;c<256;c++){
    tot++; if(c_mmed3(a,b,c)!=rs_mmed3(a,b,c)) diff++; }
  printf("exhaustive u8^3: %ld triples, %ld differ (%.2f%%)\n", tot, diff, 100.0*diff/tot);
  printf("example: a=0 b=1 c=2 -> C median %d, C2SaferRust min %d\n", c_mmed3(0,1,2), rs_mmed3(0,1,2));
  return 0; }
