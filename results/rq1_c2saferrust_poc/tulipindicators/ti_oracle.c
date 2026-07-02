/* Generic OOP oracle for tulipindicators. stdin bytes -> pick indicator via the ti_indicators[]
 * table -> build inputs/options/outputs per its counts -> call .indicator -> serialize ret + all
 * output values (NaN-canonical bit form). ASan/UBSan: nonzero exit == C UB (gate). Both C and the
 * C2SaferRust Rust side decode the SAME byte layout and read counts from the SAME table. */
#include "indicators.h"
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <unistd.h>
#include <fcntl.h>

static unsigned char _buf[1<<16]; static size_t _got, _pos;
static unsigned char nb(void){ return _pos < _got ? _buf[_pos++] : (_pos++, 0); }
static unsigned rd(int w){ unsigned v=0; for(int i=0;i<w;i++) v |= (unsigned)nb()<<(8*i); return v; }

int main(void){
    _got = fread(_buf,1,sizeof _buf,stdin); _pos=0;
    int count=0; while(ti_indicators[count].name) count++;
    int idx = (int)(nb() % (unsigned)count);
    const ti_indicator_info* T = &ti_indicators[idx];
    int size = (int)(nb() % 60) + 1;
    int ni=T->inputs, no=T->options, nout=T->outputs;

    double** inputs = (double**)malloc((ni?ni:1)*sizeof(double*));
    for(int i=0;i<ni;i++){ double* a=(double*)malloc((size_t)size*sizeof(double));
        for(int j=0;j<size;j++) a[j]=(double)rd(2)/16.0; inputs[i]=a; }
    double* opts = (double*)malloc((no?no:1)*sizeof(double));
    for(int o=0;o<no;o++) opts[o]=(double)((nb()%50)+1);   /* small positive: hits valid periods */
    double** outputs = (double**)malloc((nout?nout:1)*sizeof(double*));
    for(int k=0;k<nout;k++){ double* a=(double*)malloc((size_t)size*sizeof(double));
        memset(a,0,(size_t)size*sizeof(double)); outputs[k]=a; }

    fflush(stdout); int so=dup(1); int dn=open("/dev/null",O_WRONLY); if(dn>=0)dup2(dn,1);
    int ret = T->indicator(size, (const double*const*)inputs, opts, (double*const*)outputs);
    fflush(stdout); if(dn>=0){ dup2(so,1); close(dn);} if(so>=0) close(so);

    printf("i:%d ret:%d", idx, ret);
    for(int k=0;k<nout;k++){ printf(" o%d", k);
        for(int j=0;j<size;j++){ double v=outputs[k][j];
            if(isnan(v)) printf(":nan"); else { uint64_t b; memcpy(&b,&v,8); printf(":%llu",(unsigned long long)b); } } }
    printf("\n");

    for(int i=0;i<ni;i++) free(inputs[i]); free(inputs);
    free(opts);
    for(int k=0;k<nout;k++) free(outputs[k]); free(outputs);
    return 0;
}
