#include <ctype.h>
#include <stdio.h>
#include <string.h>

static char *lexOnce(char *p) {
  char buf[80], *b;

  b = buf;

  while (isspace(*p))
    p++;

  if (isalpha(*p)) {
    while (isalnum(*p))
      *b++ = *p++;
    *b = '\0';
    printf("%s\n", buf);
    return p;
  }

  if (isdigit(*p)) {
    while (isdigit(*p))
      *b++ = *p++;
    *b = '\0';
    printf("%s\n", buf);
    return p;
  }

  if (*p == '=') {
    printf("=\n");
    return p + 1;
  }

  return NULL;
}

static void lexLine(char *line) {
  char *p;

  p = line;
  while ((p = lexOnce(p)))
    ;
}

int main() {
  char line[80];

  while (fgets(line, sizeof(line), stdin)) {
    line[strlen(line) - 1] = '\0';
    lexLine(line);
  }
}
