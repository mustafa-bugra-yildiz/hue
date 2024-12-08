#include "ast.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void printExpr(struct expr *expr) {
  switch (expr->type) {
  case EXPR_string:
    printf("'%s'", expr->string->value);
    break;
  }
}

void printStmt(struct stmt *stmt) {
  switch (stmt->type) {
  case STMT_print:
    printf("print ");
    printExpr(stmt->print->expr);
    printf("\n");
    break;
  }
}

void printDecl(struct decl *decl) {
  switch (decl->type) {
  case DECL_fn:
    printf("fn %s:\n", decl->fn->name);
    for (int i = 0; i < decl->fn->bodyLen; i++) {
      printf("  ");
      printStmt(decl->fn->body[i]);
    }
    printf("end\n");
    break;
  }
}
