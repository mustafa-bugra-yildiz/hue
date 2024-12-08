#include "ast.h"
#include "bytecode.h"
#include "parser.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

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

void writeFile(char *path, char *contents, int length) {
  FILE *fp = fopen(path, "wb");
  assert(fp);

  fwrite(contents, 1, length, fp);
  fclose(fp);
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
  char *inputFile = NULL;
  char *outputFile = NULL;
  int shouldPrintAst = 0;
  int shouldPrintBytecode = 0;

  char c;
  while ((c = getopt(argc, argv, "i:o:ab")) != -1) {
    switch (c) {
    case 'i':
      inputFile = optarg;
      break;
    case 'o':
      outputFile = optarg;
      break;
    case 'a':
      shouldPrintAst = 1;
      break;
    case 'b':
      shouldPrintBytecode = 1;
      break;
    case '?':
      if (optopt == 'i' || optopt == 'o')
        fprintf(stderr, "Option -%c requires an argument.\n", optopt);
      else
        fprintf(stderr, "Unknown option `-%c'.\n", optopt);
      return EXIT_FAILURE;
    default:
      abort();
    }
  }

  if (inputFile == NULL) {
    fprintf(stderr, "Please specify an input file with option -i <path>");
    return EXIT_FAILURE;
  }
  if (outputFile == NULL) {
    fprintf(stderr, "Please specify an output file with option -o <path>");
    return EXIT_FAILURE;
  }

  char *code = readFile(inputFile);
  assert(code);

  struct decl *decl = parseDeclFn(&code);
  if (shouldPrintAst) {
    printf("\n--- Parsed Code ---\n");
    printDecl(decl);
  }

  struct bytecode *bc = makeBytecode();
  assert(bc);

  lowerDecl(bc, decl);

  if (shouldPrintBytecode) {
    printf("\n--- Compiled Bytecode ---\n");
    printBytecode(bc);
    putchar('\n');
  }

  int length;
  char *serializedBytecode = serializeBytecode(bc, &length);
  writeFile(outputFile, serializedBytecode, length);
  return 0;
}
