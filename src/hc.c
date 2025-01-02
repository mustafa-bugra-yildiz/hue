#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define LINE_SIZE 80
#define STACK_SIZE 80
#define RULES_SIZE 80

enum type {
  N_END, // end of pattern

  /* nodes */
  N_FN,
  N_ADD,
  N_EXPR,

  /* tokens */
  N_SEMI,
  N_IDENT,
  N_NUMBER,
  N_EQ,
  N_PLUS,
};

struct node {
  enum type t;
  union {
    struct {
      char *ident;
      struct node *number;
    } fn;

    struct {
      struct node *lhs, *rhs;
    } add;

    struct node *expr;

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
int reg;

static void indent(int level) {
  int i = 0;
  while (i++ < level)
    printf("  ");
}

static void printNode(struct node *n, int level) {
  switch (n->t) {
  case N_FN:
    indent(level), printf("fn %s\n", n->fn.ident);
    printNode(n->fn.number, level + 1);
    break;

  case N_IDENT:
    indent(level), printf("ident %s\n", n->ident);
    break;

  case N_EQ:
    indent(level), printf("eq\n");
    break;

  case N_PLUS:
    indent(level), printf("plus\n");
    break;

  case N_SEMI:
    indent(level), printf("semi\n");
    break;

  case N_NUMBER:
    indent(level), printf("number %d\n", n->number);
    break;

  case N_ADD:
    indent(level), printf("add\n");
    printNode(n->add.lhs, level + 1);
    printNode(n->add.rhs, level + 1);
    break;

  case N_EXPR:
    printNode(n->expr, level);
    break;

  case N_END: /* do nothing */
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

  if (*tok == '+') {
    n = mkNode(N_PLUS);
    return n;
  }

  if (*tok == ';') {
    n = mkNode(N_SEMI);
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

static struct node *mkAdd(struct node **args) {
  struct node *expr, *add, *lhs, *rhs;

  lhs = args[0];
  rhs = args[2];

  add = mkNode(N_ADD);
  add->add.lhs = lhs;
  add->add.rhs = rhs;

  expr = mkNode(N_EXPR);
  expr->expr = add;

  return expr;
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

static int compile(struct node *n, int level) {
  int lhs, rhs;

  switch (n->t) {
  case N_FN:
    reg = 0;

    indent(level), printf("%s:\n", n->fn.ident);
    compile(n->fn.number, level + 1);
    indent(level + 1), printf("ret\n");

    return reg;

  case N_ADD:
    lhs = compile(n->add.lhs, level);
    rhs = compile(n->add.rhs, level);
    indent(level), printf("add x%d, x%d, x%d\n", lhs, lhs, rhs);
    return lhs;

  case N_EXPR:
    return compile(n->expr, level);

  case N_NUMBER:
    indent(level), printf("mov x%d, #%d\n", reg, n->number);
    return reg++;

  default:
    fprintf(stderr, "error: cannot compile node type %d\n", n->t);
    exit(1);
  }
}

int main(int argc, char *argv[]) {
  struct node **np;
  int opt;
  char mode;

  /* opts */
  mode = 'c'; /* compile */
  while ((opt = getopt(argc, argv, "p"))) {
    switch (opt) {
    case -1:
      goto GETOPT_DONE;

    case 'p':
      mode = 'd'; /* debug ast */
      break;

    default:
      fprintf(stderr, "error: unknown option %c\n", opt);
      return 1;
    }
  }
GETOPT_DONE:

  /* init globals */
  stack = calloc(STACK_SIZE, sizeof(stack[0]));
  sp = stack;

  rules = calloc(RULES_SIZE, sizeof(rules[0]));
  rp = rules;

  /* function rules */
  *rp++ = (struct rule){mkFn, {N_IDENT, N_EQ, N_EXPR, N_SEMI}};
  *rp++ = (struct rule){mkFn, {N_IDENT, N_EQ, N_ADD, N_SEMI}};
  *rp++ = (struct rule){mkFn, {N_IDENT, N_EQ, N_NUMBER, N_SEMI}};

  /* expr rules */
  *rp++ = (struct rule){mkAdd, {N_NUMBER, N_PLUS, N_NUMBER}};

  while (push())
    reduce();

  np = stack;

  if (mode == 'c') {
    while (np != sp) {
      compile(*np, 0);
      np++;
    }
  }

  if (mode == 'd') {
    while (np != sp) {
      printNode(*np, 0);
      np++;
    }
  }
}
