#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "url.h"

/* Deterministic argv driver for urlparser (jwerle url.h, header-only).
 * argv[1] = URL. Exercises every public function in url.h. */
static void show(const char *label, char *v) {
  printf("%s: %s\n", label, v ? v : "(null)");
  if (v) free(v);
}

int main(int argc, char **argv) {
  if (argc < 3) { printf("usage: driver <url> <scheme-word>\n"); return 1; }
  char *url = argv[1];
  char *word = argv[2];
  printf("is_protocol(%s): %d\n", word, (int)url_is_protocol(word));
  printf("is_ssh(%s): %d\n", word, (int)url_is_ssh(word));
  show("protocol", url_get_protocol(url));
  show("auth", url_get_auth(url));
  show("hostname", url_get_hostname(url));
  show("host", url_get_host(url));
  show("pathname", url_get_pathname(url));
  show("path", url_get_path(url));
  show("search", url_get_search(url));
  show("query", url_get_query(url));
  show("hash", url_get_hash(url));
  show("port", url_get_port(url));
  url_data_t *d = url_parse(url);
  if (d) { url_data_inspect(d); url_free(d); }
  else printf("parse: NULL\n");
  url_inspect(url);
  return 0;
}
