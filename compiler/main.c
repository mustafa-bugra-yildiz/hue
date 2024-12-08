#include "ast.h"
#include "bytecode.h"
#include "parser.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *readFile(char *path) {
  FILE *fp = fopen(path, "r");
  assert(fp);

  fseek(fp, 0, SEEK_END);
  int size = ftell(fp);
  rewind(fp);

  char *buffer = malloc(size + 1);
  fread(buffer, 1, size, fp);

  fclose(fp);
  return buffer;
}

void lowerExpr(struct bytecode *bc, struct fn *fn, struct expr *expr) {
  int index;

  switch (expr->type) {
  case EXPR_string:
    index = appendLiteral(bc, makeObjectString(expr->string->value));
    appendInst(fn, (struct inst){
                       .type = INST_loadLit,
                       .arg = index,
                   });
    break;
  }
}

void lowerStmt(struct bytecode *bc, struct fn *fn, struct stmt *stmt) {
  switch (stmt->type) {
  case STMT_print:
    lowerExpr(bc, fn, stmt->print->expr);
    appendInst(fn, (struct inst){.type = INST_print, .arg = 0});
    break;
  }
}

void lowerDecl(struct bytecode *bc, struct decl *decl) {
  struct fn *fn;

  switch (decl->type) {
  case DECL_fn:
    fn = makeFn(decl->fn->name);
    assert(fn);

    for (int i = 0; i < decl->fn->bodyLen; i++)
      lowerStmt(bc, fn, decl->fn->body[i]);

    if (strcmp(fn->name, "main") == 0)
      appendInst(fn, (struct inst){.type = INST_halt, .arg = 0});

    appendFn(bc, fn);
    break;
  }
}

int main(int argc, char *argv[]) {
  argc--;
  argv++;

  assert(argc == 1);

  char *code = readFile(argv[0]);
  assert(code);

  struct decl *decl = parseDeclFn(&code);
  printf("\n--- Parsed Code ---\n");
  printDecl(decl);

  struct bytecode *bc = makeBytecode();
  assert(bc);

  printf("\n--- Compiled Bytecode ---\n");
  lowerDecl(bc, decl);
  printBytecode(bc);
  putchar('\n');

  return 0;
}
