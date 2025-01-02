#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_SIZE 80
#define STACK_SIZE 80
#define RULES_SIZE 80

enum type {
  N_END, // end of pattern

  N_FN,
  N_IDENT,
  N_EQ,
  N_NUMBER,
};

struct node {
  enum type t;
  union {
    struct {
      char *ident;
      struct node *number;
    } fn;

    char *ident;
    int number;
  };
};

struct rule {
  struct node *(*mker)(struct node **);
  enum type pattern[80];
};

struct node **stack, **sp;
struct rule *rules, *rp;

static void indent(int level) {
  int i = 0;
  while (i++ < level)
    printf("  ");
}

static void printNode(struct node *n, int level) {
  indent(level);

  switch (n->t) {
  case N_END: /* do nothing */
    break;
  case N_FN:
    printf("fn %s\n", n->fn.ident);
    printNode(n->fn.number, level + 1);
    break;
  case N_IDENT:
    printf("ident %s\n", n->ident);
    break;
  case N_EQ:
    printf("eq\n");
    break;
  case N_NUMBER:
    printf("number %d\n", n->number);
    break;
  }
}

static int pLenFn(enum type *pattern) {
  enum type *pp;
  int len;

  pp = pattern;
  len = 0;
  while (*pp != N_END)
    pp++, len++;

  return len;
}

static struct node *mkNode(enum type t) {
  struct node *n;
  n = malloc(sizeof(*n));
  n->t = t;
  return n;
}

static struct node *mkTok(char *tok) {
  struct node *n;

  if (isalpha(*tok)) {
    n = mkNode(N_IDENT);
    n->ident = tok;
    return n;
  }

  if (isdigit(*tok)) {
    n = mkNode(N_NUMBER);
    n->number = atoi(tok);
    return n;
  }

  if (*tok == '=') {
    n = mkNode(N_EQ);
    return n;
  }

  fprintf(stderr, "error: unknown token '%s'\n", tok);
  exit(1);
}

static int patternMatches(enum type *pattern) {
  int sLen, pLen, i;

  pLen = pLenFn(pattern);
  sLen = sp - stack;

  /* Pattern is longer than stack, doesn't match */
  if (pLen > sLen)
    return 0;

  /* Check pattern matches 1-1 */
  for (i = pLen - 1; i >= 0; i--) {
    if (pattern[i] != stack[sLen - pLen + i]->t)
      return 0;
  }

  /* Pattern matches */
  return 1;
}

static struct node *mkFn(struct node **args) {
  struct node *fn, *ident, *number;

  ident = args[0];
  number = args[2];

  fn = mkNode(N_FN);
  fn->fn.ident = ident->ident;
  fn->fn.number = number;

  return fn;
}

static int push() {
  struct node *n;
  char *tok;

  tok = malloc(LINE_SIZE);
  tok = fgets(tok, LINE_SIZE, stdin);
  if (!tok)
    return 0;
  tok[strlen(tok) - 1] = '\0';

  n = mkTok(tok);
  *sp++ = n;

  return 1;
}

static void reduce() {
  struct rule *r;
  struct node *n, *args[80];
  int matches, pLen, i;

  r = rules;
  while (r != rp) {
    matches = patternMatches(r->pattern);
    if (!matches) {
      r++;
      continue;
    }

    pLen = pLenFn(r->pattern);
    for (i = pLen - 1; i >= 0; i--) {
      args[i] = *--sp;
    }

    n = r->mker(args);
    *sp++ = n;

    r++;
  }
}

static void compile(struct node *n, int level) {
  switch (n->t) {
  case N_FN:
    indent(level), printf("%s:\n", n->fn.ident);
    compile(n->fn.number, level + 1);
    indent(level + 1), printf("ret\n");
    break;

  case N_NUMBER:
    indent(level), printf("mov x0, #%d\n", n->number);
    break;

  default:
    fprintf(stderr, "error: cannot compile node type %d\n", n->t);
    exit(1);
  }
}

int main() {
  struct node **np;

  stack = calloc(STACK_SIZE, sizeof(stack[0]));
  sp = stack;

  rules = calloc(RULES_SIZE, sizeof(rules[0]));
  rp = rules;

  *rp++ = (struct rule){mkFn, {N_IDENT, N_EQ, N_NUMBER}};

  while (push())
    reduce();

  np = stack;
  while (np != sp) {
    compile(*np, 0);
    np++;
  }
}
