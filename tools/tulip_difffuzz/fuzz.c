/*
 * Differential fuzzer for Tulip Indicators: C oracle vs a Rust translation
 * (c2rust baseline, or C2SaferRust's LLM-rewritten output), over the uniform
 * indicator ABI. One harness covers all 104 indicators.
 *
 * The Rust lib's `ti_*` symbols are renamed to `r_ti_*` (objcopy --redefine-syms)
 * and force-linked (--whole-archive) so both the C oracle (ti_*) and the Rust
 * translation (r_ti_*) live in one binary; the Rust fn is resolved by name via
 * dlsym(RTLD_DEFAULT,"r_ti_<name>") (build with -rdynamic).
 *
 * ROBUST DRIVER: each indicator call (C and Rust) runs under sigsetjmp; a shared
 * handler for SIGSEGV/SIGFPE/SIGABRT/SIGBUS and the macOS-style __assert_rtn
 * longjmp back, recording WHICH side trapped. A trap on the Rust side when C
 * returned OKAY is a real divergence (crash/assert introduced by the translation).
 *
 * Run: ./fuzz_<which> <seed> <iters_per_indicator> [reltol]
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>
#include <dlfcn.h>
#include <setjmp.h>
#include <signal.h>
#include <unistd.h>
#include "indicators.h"   /* ti_indicator_info, ti_indicators[], TI_OKAY, TI_MAXINDPARAMS, TI_REAL */

/* ---- trap handling shared by C and Rust calls ---- */
static sigjmp_buf g_jb;
static volatile sig_atomic_t g_armed = 0;
static volatile int g_trapcode = 0;     /* signal number, or -1 for assert */
static void on_signal(int sig){ if (g_armed){ g_armed=0; g_trapcode=sig; siglongjmp(g_jb,1);} else { _exit(128+sig); } }
/* macOS assert symbol referenced by the macOS-generated c2rust crates */
void __assert_rtn(const char* fn,const char* f,int l,const char* e){
    (void)fn;(void)f;(void)l;(void)e;
    if (g_armed){ g_armed=0; g_trapcode=-1; siglongjmp(g_jb,1); }
    fprintf(stderr,"assert outside guarded call: %s:%d %s\n", f?f:"?", l, e?e:"?"); abort();
}
static void install_handlers(void){
    struct sigaction sa; memset(&sa,0,sizeof sa);
    sa.sa_handler=on_signal; sigemptyset(&sa.sa_mask); sa.sa_flags=SA_NODEFER;
    sigaction(SIGSEGV,&sa,0); sigaction(SIGFPE,&sa,0); sigaction(SIGABRT,&sa,0); sigaction(SIGBUS,&sa,0);
}
static const char* trapname(int c){
    switch(c){case -1:return "assert";case SIGSEGV:return "SIGSEGV";case SIGFPE:return "SIGFPE";
              case SIGABRT:return "SIGABRT";case SIGBUS:return "SIGBUS";default:return "signal";}
}

/* ---- reproducible RNG ---- */
static uint64_t RNG = 0x9e3779b97f4a7c15ULL;
static inline uint64_t xrand(void){ RNG^=RNG<<13; RNG^=RNG>>7; RNG^=RNG<<17; return RNG; }
static inline double urand(double lo,double hi){ return lo+(hi-lo)*((double)(xrand()>>11)/(double)(1ULL<<53)); }

#define MAXN 64
#define MAXP TI_MAXINDPARAMS
static int g_reltol_set=0; static double g_reltol=0.0;

static int diverges(double a,double b){
    if (isnan(a)||isnan(b)) return isnan(a)!=isnan(b);
    if (isinf(a)||isinf(b)) return !(isinf(a)&&isinf(b)&&((a>0)==(b>0)));
    if (a==b) return 0;
    if (!g_reltol_set||g_reltol==0.0) return 1;
    double d=fabs(a)>fabs(b)?fabs(a):fabs(b);
    if (d<1e-300) return fabs(a-b)>1e-12;
    return fabs(a-b)/d > g_reltol;
}
static void make_options(const ti_indicator_info *info,int n,TI_REAL *opt){
    for (int i=0;i<info->options;i++){
        const char *nm=info->option_names[i]?info->option_names[i]:"";
        if (strstr(nm,"period")||strstr(nm,"Period")){ int hi=n/2; if(hi<2)hi=2;
            opt[i]=(double)(2+(int)(xrand()%(uint64_t)hi)); }
        else if (strstr(nm,"stddev")||strstr(nm,"factor")||strstr(nm,"deviations")) opt[i]=urand(0.5,4.0);
        else opt[i]=(xrand()&1)?(double)(1+(int)(xrand()%10)):urand(0.1,5.0);
    }
}

int main(int argc,char**argv){
    uint64_t seed=(argc>1)?strtoull(argv[1],0,10):1;
    long iters=(argc>2)?strtol(argv[2],0,10):20000;
    if (argc>3){ g_reltol=strtod(argv[3],0); g_reltol_set=1; }
    RNG=seed?seed:1;
    install_handlers();

    long total=0,compared=0,c_err=0,c_trap=0,rc_div=0,val_div=0,rust_trap=0;
    int buggy=0;

    static TI_REAL buf[MAXP][MAXN], oc[MAXP][MAXN], orr[MAXP][MAXN];
    const TI_REAL *inputs[MAXP]; TI_REAL *out_c[MAXP], *out_r[MAXP];

    for (int idx=0; ti_indicators[idx].name; idx++){
        const ti_indicator_info *ci=&ti_indicators[idx];
        if (!ci->indicator) continue;
        char sym[160]; snprintf(sym,sizeof sym,"r_ti_%s",ci->name);
        ti_indicator_function rfn=(ti_indicator_function)dlsym(RTLD_DEFAULT,sym);
        if (!rfn){ fprintf(stderr,"[skip] no rust symbol %s\n",sym); continue; }
        int reported=0;

        for (long it=0; it<iters; it++){
            total++;
            int n=2+(int)(xrand()%(MAXN-2));
            for (int k=0;k<ci->inputs;k++){
                for (int j=0;j<n;j++){ double base=urand(1.0,1000.0);
                    buf[k][j]=(xrand()%8==0)?urand(-1000.0,1000.0):base; }
                inputs[k]=buf[k];
            }
            TI_REAL options[MAXP]; memset(options,0,sizeof options); make_options(ci,n,options);
            for (int k=0;k<ci->outputs;k++){ for(int j=0;j<MAXN;j++){oc[k][j]=0.0;orr[k][j]=-987654.321;}
                out_c[k]=oc[k]; out_r[k]=orr[k]; }

            /* --- C oracle (guarded) --- */
            int rc_c;
            if (sigsetjmp(g_jb,1)==0){ g_armed=1; rc_c=ci->indicator(n,inputs,options,out_c); g_armed=0; }
            else { g_armed=0; c_trap++; continue; }      /* C itself trapped -> bad input for C, skip */
            if (rc_c!=TI_OKAY){ c_err++; continue; }

            /* --- Rust translation (guarded) --- */
            int rc_r;
            if (sigsetjmp(g_jb,1)==0){ g_armed=1; rc_r=rfn(n,inputs,options,out_r); g_armed=0; }
            else { g_armed=0; rust_trap++;
                if (!reported){ printf("DIVERGENCE [%s] Rust %s where C returned OKAY (n=%d, opts:",
                                        ci->name, trapname(g_trapcode), n);
                    for(int o=0;o<ci->options;o++) printf(" %.6g",options[o]); printf(")\n"); reported=1; }
                continue;
            }
            compared++;

            if (rc_r!=rc_c){ rc_div++;
                if (!reported){ printf("DIVERGENCE [%s] return code C=%d Rust=%d (n=%d)\n",ci->name,rc_c,rc_r,n); reported=1; }
                continue;
            }
            int start=0;
            if (sigsetjmp(g_jb,1)==0){ g_armed=1; start=ci->start?ci->start(options):0; g_armed=0; } else g_armed=0;
            int outlen=n-start; if(outlen<0)outlen=0; if(outlen>MAXN)outlen=MAXN;
            int found=0;
            for (int k=0;k<ci->outputs&&!found;k++) for (int j=0;j<outlen;j++)
                if (diverges(out_c[k][j],out_r[k][j])){ val_div++;
                    if (!reported){ printf("DIVERGENCE [%s] output[%d][%d] C=%.17g Rust=%.17g (n=%d, opts:",
                                            ci->name,k,j,out_c[k][j],out_r[k][j],n);
                        for(int o=0;o<ci->options;o++) printf(" %.6g",options[o]); printf(")\n"); reported=1; }
                    found=1; break; }
        }
        if (reported) buggy++;
    }
    printf("\n=== summary (seed=%llu iters/ind=%ld reltol=%g) ===\n",(unsigned long long)seed,iters,g_reltol_set?g_reltol:0.0);
    printf("total=%ld compared=%ld c_invalid=%ld c_trap=%ld\n",total,compared,c_err,c_trap);
    printf("rust_traps=%ld return_code_div=%ld value_div=%ld  buggy_indicators=%d\n",rust_trap,rc_div,val_div,buggy);
    return buggy?1:0;
}
