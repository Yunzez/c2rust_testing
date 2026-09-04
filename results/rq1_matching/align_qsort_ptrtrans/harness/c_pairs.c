#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "qsort.c"
/* op: swap | partition | quicksort ; input: T then per case */
int main(int argc,char**argv){
    const char*op=argv[1];
    int T; if(scanf("%d",&T)!=1) return 2;
    for(int t=0;t<T;t++){
        if(!strcmp(op,"swap")){ int a,b; scanf("%d %d",&a,&b); swap(&a,&b); printf("%d %d\n",a,b); continue; }
        int n,low,high; scanf("%d %d %d",&n,&low,&high);
        int *a = malloc(sizeof(int)*(n>0?n:1));
        for(int i=0;i<n;i++) scanf("%d",&a[i]);
        if(!strcmp(op,"partition")){ int r=partition(a,low,high); printf("ret=%d :",r); }
        else quickSort(a,low,high);
        for(int i=0;i<n;i++) printf(" %d",a[i]);
        printf("\n"); free(a);
    }
    return 0;
}
