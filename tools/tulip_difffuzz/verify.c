#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
typedef double TI_REAL;
void __assert_rtn(const char*a,const char*b,int c,const char*d){(void)a;(void)b;(void)c;(void)d;}
extern int ti_adx_start(const TI_REAL*);
int main(void){
    int (*r_start)(const TI_REAL*) = (int(*)(const TI_REAL*))dlsym(RTLD_DEFAULT,"r_ti_adx_start");
    for (double p = 2; p <= 20; p += 6){
        TI_REAL opt[1] = { p };
        int c = ti_adx_start(opt);
        int r = r_start ? r_start(opt) : -999999;
        printf("period=%2.0f :  C ti_adx_start = %-4d |  C2SaferRust = %-12d %s\n",
               p, c, r, (c==r?"":"<<< DIVERGE"));
    }
    return 0;
}
