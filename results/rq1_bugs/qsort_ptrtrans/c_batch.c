#include <stdio.h>
#include <stdlib.h>
#include "qsort.c"
int main(void){
    int T; if(scanf("%d",&T)!=1) return 2;
    for(int t=0;t<T;t++){
        int n; scanf("%d",&n);
        int *a = malloc(sizeof(int)*(n>0?n:1));
        for(int i=0;i<n;i++) scanf("%d",&a[i]);
        quickSort(a,0,n-1);
        for(int i=0;i<n;i++) printf("%d ",a[i]);
        printf("\n");
        free(a);
    }
    return 0;
}
