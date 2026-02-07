#include "checkpoint.h"
#include <stdio.h>

void cp(const char *label, const void *ptr, size_t len) {
    // Minimal trace: label + length + pointer value.
    // For real use, you would hash memory instead of printing pointers.
    fprintf(stderr, "CP %s len=%zu ptr=%p\n", label, len, ptr);
}
