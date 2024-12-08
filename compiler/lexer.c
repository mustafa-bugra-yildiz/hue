#include "lexer.h"

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct token *makeToken(enum tokenType type, char *value) {
  struct token *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->type = type;
  ptr->value = value;

  return ptr;
}

struct token *nextToken(char **base) {
  char *i = *base;

  while (isspace(*i))
    i++;

  /* Keywords & symbols */
  if (isalpha(*i)) {
    char *value = malloc(80);
    assert(value);

    char *v = value;
    while (isalnum(*i))
      *v++ = *i++;
    *v = '\0';

    *base = i;
    if (strcmp(value, "fn") == 0)
      return makeToken(TOKEN_fn, value);
    if (strcmp(value, "end") == 0)
      return makeToken(TOKEN_end, value);
    if (strcmp(value, "print") == 0)
      return makeToken(TOKEN_print, value);
    return makeToken(TOKEN_symbol, value);
  }

  /* Strings */
  if (*i == '"' || *i == '\'') {
    char pair = *i++;

    char *value = malloc(80);
    assert(value);

    char *v = value;
    while (*i != pair)
      *v++ = *i++;
    *v = '\0';
    i++;

    *base = i;
    return makeToken(TOKEN_string, value);
  }

  /* Symbols */
  if (*i == ':') {
    *base = i + 1;
    return makeToken(TOKEN_colon, strdup(":"));
  }

  /* EOF */
  if (*i == '\0') {
    return NULL;
  }

  /* Unknown char */
  fprintf(stderr, "error: unexpected character '%c'\n", *i);
  exit(EXIT_FAILURE);
}

struct token *eatToken(enum tokenType type, char **base) {
  struct token *next = nextToken(base);
  if (next == NULL) {
    fprintf(stderr, "error: expected token type %d, found NULL\n", type);
    exit(EXIT_FAILURE);
  }
  if (next->type != type) {
    fprintf(stderr, "error: expected token type %d, found %d\n", type,
            next->type);
    exit(EXIT_FAILURE);
  }
  return next;
}

struct token *tryEatingToken(enum tokenType type, char **base) {
  char *iter = *base;
  struct token *next = nextToken(&iter);
  if (next == NULL)
    return NULL;
  if (next->type != type)
    return NULL;
  *base = iter;
  return next;
}
