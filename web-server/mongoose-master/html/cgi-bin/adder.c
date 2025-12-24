#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int parse_two_ints_from_query(const char *qs, long *a, long *b) {
  // Assignment example: "1500&212"
  if (qs == NULL || *qs == '\0') return 0;

  // Copy because we'll mutate
  char buf[256];
  snprintf(buf, sizeof(buf), "%s", qs);

  char *p1 = buf;
  char *amp = strchr(buf, '&');
  if (amp == NULL) return 0;
  *amp = '\0';
  char *p2 = amp + 1;

  char *end1 = NULL;
  char *end2 = NULL;
  long v1 = strtol(p1, &end1, 10);
  long v2 = strtol(p2, &end2, 10);

  if (end1 == p1 || end2 == p2) return 0;
  *a = v1;
  *b = v2;
  return 1;
}

int main(void) {
  long a = 0, b = 0;
  const char *qs = getenv("QUERY_STRING");

  int ok = parse_two_ints_from_query(qs, &a, &b);
  if (!ok) {
    // Fallback: read from stdin (works better for POST, but kept for assignment wording)
    if (scanf("%ld%ld", &a, &b) != 2) {
      // Output a valid HTTP response anyway
      printf("Content-Type: text/plain; charset=utf-8\r\n\r\n");
      printf("bad input (QUERY_STRING='%s')\n", qs ? qs : "");
      return 0;
    }
  }

  printf("Content-Type: text/plain; charset=utf-8\r\n\r\n");
  printf("%ld+%ld=%ld\n", a, b, a + b);
  return 0;
}
