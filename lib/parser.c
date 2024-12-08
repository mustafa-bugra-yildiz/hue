#include "parser.h"

#include "ast.h"
#include "lexer.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

static struct expr *makeExprString(char *value) {
  assert(value);

  struct exprString *variant = malloc(sizeof(*variant));
  assert(variant);

  variant->value = value;

  struct expr *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->type = EXPR_string;
  ptr->string = variant;

  return ptr;
};

static struct expr *parseExprString(char **base) {
  struct token *token = tryEatingToken(TOKEN_string, base);
  if (token == NULL)
    return NULL;
  return makeExprString(token->value);
}

static struct stmt *makeStmtPrint(struct expr *expr) {
  assert(expr);

  struct stmtPrint *variant = malloc(sizeof(*variant));
  assert(variant);

  variant->expr = expr;

  struct stmt *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->type = STMT_print;
  ptr->print = variant;

  return ptr;
}

static struct stmt *parseStmtPrint(char **base) {
  struct token *print = tryEatingToken(TOKEN_print, base);
  if (print == NULL)
    return NULL;

  struct expr *expr = parseExprString(base);
  assert(expr);

  return makeStmtPrint(expr);
}

static struct declFn *makeDeclFn(char *name) {
  assert(name);

  struct declFn *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->name = name;

  ptr->bodyLen = 0;
  ptr->bodyCap = 10;
  ptr->body = malloc(ptr->bodyCap * sizeof(ptr->body[0]));
  assert(ptr->body);

  return ptr;
}

static void appendStmt(struct declFn *declFn, struct stmt *stmt) {
  assert(declFn);
  assert(stmt);

  if (declFn->bodyLen == declFn->bodyCap) {
    declFn->bodyCap *= 2;
    declFn->body =
        realloc(declFn->body, declFn->bodyCap * sizeof(declFn->body[0]));
    assert(declFn->body);
  }

  declFn->body[declFn->bodyLen++] = stmt;
}

struct decl *parseDeclFn(char **base) {
  struct token *fn = tryEatingToken(TOKEN_fn, base);
  if (fn == NULL)
    return NULL;

  struct token *name = eatToken(TOKEN_symbol, base);
  assert(name);

  struct token *colon = eatToken(TOKEN_colon, base);
  assert(colon);

  struct declFn *declFn = makeDeclFn(name->value);
  assert(declFn);

  struct stmt *stmt = parseStmtPrint(base);
  assert(stmt);

  appendStmt(declFn, stmt);

  struct token *end = eatToken(TOKEN_end, base);
  assert(end);

  struct decl *decl = malloc(sizeof(*decl));
  assert(decl);

  decl->type = DECL_fn;
  decl->fn = declFn;

  return decl;
}
