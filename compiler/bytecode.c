#include "bytecode.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

struct fn *makeFn(char *name) {
  assert(name);

  struct fn *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->name = name;

  ptr->bodyLen = 0;
  ptr->bodyCap = 10;
  ptr->body = malloc(ptr->bodyCap * sizeof(ptr->body[0]));
  assert(ptr->body);

  return ptr;
}

void appendInst(struct fn *fn, struct inst inst) {
  assert(fn);

  if (fn->bodyCap == fn->bodyLen) {
    fn->bodyCap *= 2;
    fn->body = realloc(fn->body, fn->bodyCap * sizeof(fn->body[0]));
    assert(fn);
  }

  fn->body[fn->bodyLen++] = inst;
}

struct bytecode *makeBytecode() {
  struct bytecode *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->literalsLen = 0;
  ptr->literalsCap = 10;
  ptr->literals = malloc(ptr->literalsCap * sizeof(ptr->literals[0]));
  assert(ptr->literals);

  ptr->fnsLen = 0;
  ptr->fnsCap = 10;
  ptr->fns = malloc(ptr->fnsCap * sizeof(ptr->fns[0]));
  assert(ptr->fns);

  return ptr;
}

void printBytecode(struct bytecode *bc) {
  assert(bc);

  printf("literals:\n");
  for (int i = 0; i < bc->literalsLen; i++) {
    printf("- %d: ", i);

    struct object *object = bc->literals[i];
    switch (object->type) {
    case OBJECT_string:
      printf("string '%s'\n", object->string);
      break;
    }
  }

  printf("\nfns:\n");
  for (int i = 0; i < bc->fnsLen; i++) {
    struct fn *fn = bc->fns[i];

    printf("- %s:\n", fn->name);
    for (int j = 0; j < fn->bodyLen; j++) {
      struct inst inst = fn->body[j];

      printf("  ");
      switch (inst.type) {
      case INST_loadLit:
        printf("LOAD_LIT %d\n", inst.arg);
        break;

      case INST_print:
        printf("PRINT\n");
        break;

      case INST_halt:
        printf("HALT\n");
        break;
      }
    }
  }
}

int appendLiteral(struct bytecode *bc, struct object *object) {
  assert(bc);
  assert(object);

  if (bc->literalsCap == bc->literalsLen) {
    bc->literalsCap *= 2;
    bc->literals = malloc(bc->literalsCap * sizeof(bc->literals[0]));
    assert(bc->literals);
  }

  bc->literals[bc->literalsLen++] = object;
  return bc->literalsLen - 1;
}

void appendFn(struct bytecode *bc, struct fn *fn) {
  assert(bc);
  assert(fn);

  if (bc->fnsCap == bc->fnsLen) {
    bc->fnsCap *= 2;
    bc->fns = malloc(bc->fnsCap * sizeof(bc->fns[0]));
    assert(bc->fns);
  }

  bc->fns[bc->fnsLen++] = fn;
}
