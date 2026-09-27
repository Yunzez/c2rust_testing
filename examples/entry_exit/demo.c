#include "checkpoint.h"
#include <string.h>

// Simple MTU: reverse a string in place.
void reverse(char *s) {
    cp("entry", s, s ? strlen(s) : 0);

    if (!s) {
        cp("exit", s, 0);
        return;
    }

    size_t n = strlen(s);
    for (size_t i = 0; i < n / 2; i++) {
        char tmp = s[i];
        s[i] = s[n - 1 - i];
        s[n - 1 - i] = tmp;
    }

    cp("exit", s, n);
}

int main(void) {
    char buf[] = "abcd";
    reverse(buf);
    return 0;
}
