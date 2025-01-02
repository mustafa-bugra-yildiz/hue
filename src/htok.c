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

  if (*p == '-' && *(p + 1) == '>') {
    printf("->");
    return p + 2;
  }

  if (*p == '=' || *p == '+') {
    printf("%c\n", *p);
    return p + 1;
  }

  return NULL;
}

static int lexLine(char *line) {
  char *p;
  int toks;

  toks = 0;
  p = line;

  while ((p = lexOnce(p)))
    toks++;

  return toks;
}

int main() {
  char line[80];
  int toks;

  while (fgets(line, sizeof(line), stdin)) {
    line[strlen(line) - 1] = '\0';
    toks = lexLine(line);
    if (toks)
      printf(";\n");
  }
}
