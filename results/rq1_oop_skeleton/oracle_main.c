/* Out-of-process differential ORACLE for quickSort.
 *
 * Reads raw bytes from stdin, decodes them into an int array with the SAME format the
 * Rust harness uses (byte0 = n mod 33; then n little-endian i32), calls the real C
 * quickSort(arr, 0, n-1), and prints "n v0 v1 ... v(n-1)\n" to stdout.
 *
 * Compiled with UBSan (+ASan): if the C reference hits UB on this input the sanitizer
 * aborts with a nonzero exit -> the harness treats that as "C is UB here" and discards
 * the input (the UB gate). A clean run exits 0 with the sorted array on stdout.
 *
 * The C algorithm itself is #include'd verbatim from qsort.c (the oracle == the real C). */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

#include "qsort.c"   /* swap / partition / quickSort, byte-for-byte */

#define MAXLEN 32

int main(void) {
    unsigned char buf[1 << 16];
    size_t got = fread(buf, 1, sizeof buf, stdin);
    int n = 0;
    if (got >= 1) n = (int)(buf[0] % (MAXLEN + 1));   /* 0..32 elements */

    int arr[MAXLEN];
    for (int i = 0; i < n; i++) {
        size_t off = 1 + (size_t)i * 4;
        uint32_t v = 0;
        for (int b = 0; b < 4; b++) {
            unsigned char byte = (off + b < got) ? buf[off + b] : 0;  /* zero-fill past end */
            v |= (uint32_t)byte << (8 * b);                            /* little-endian */
        }
        arr[i] = (int32_t)v;
    }

    if (n > 0) quickSort(arr, 0, n - 1);

    printf("%d", n);
    for (int i = 0; i < n; i++) printf(" %d", arr[i]);
    printf("\n");
    return 0;
}
