#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

bool url_is_ssh_valid(int8_t value[16]);

bool
url_is_ssh (char *str) {
  str = strdup(str);
  if (0 == strcmp(str, "ssh") ||
      0 == strcmp(str, "git")) {
    free(str);
    return true;

  }

  return false;
}

bool url_is_ssh_valid(int8_t value[16]) { value[15] = 0; return url_is_ssh((char *)value); }
