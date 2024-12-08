#include "bytecode.h"

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

void evalFn(struct bytecode *bc, struct fn *fn) {
  struct object *stack[80];
  struct object **sp = stack;
  struct object *object;

  int pc = 0;

  while (fn->body[pc].type != INST_halt) {
    struct inst inst = fn->body[pc++];
    switch (inst.type) {
    case INST_loadLit:
      *sp++ = bc->literals[inst.arg];
      break;

    case INST_print:
      object = *--sp;
      switch (object->type) {
      case OBJECT_string:
        printf("%s\n", object->string);
        break;
      }
      break;

    case INST_halt:
      break;
    }
  }
}

void eval(struct bytecode *bc) {
  assert(bc);

  struct fn *mainFn = NULL;

  for (int i = 0; i < bc->fnsLen; i++) {
    struct fn *fn = bc->fns[i];
    if (strcmp(fn->name, "main") == 0) {
      mainFn = fn;
      break;
    }
  }

  if (mainFn == NULL) {
    fprintf(stderr, "error: Could not find the main function\n");
    exit(EXIT_FAILURE);
  }

  evalFn(bc, mainFn);
}

int main(int argc, char *argv[]) {
  char *inputFile = NULL;
  int shouldPrintBytecode = 0;

  char c;
  while ((c = getopt(argc, argv, "bi:")) != -1) {
    switch (c) {
    case 'i':
      inputFile = optarg;
      break;
    case 'b':
      shouldPrintBytecode = 1;
      break;
    case '?':
      if (optopt == 'i')
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

  char *serializedBytecode = readFile(inputFile);
  assert(serializedBytecode);

  struct bytecode *bc = deserializeBytecode(serializedBytecode);
  if (shouldPrintBytecode)
    printBytecode(bc);

  eval(bc);
  return 0;
}
