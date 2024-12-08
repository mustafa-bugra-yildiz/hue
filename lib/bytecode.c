#include "bytecode.h"
#include "object.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

char *appendBytes(char *dst, char *src, int len) {
  int i = 0;
  while (i++ < len) {
    *dst++ = *src++;
  }
  return dst;
}

char *serializeBytecode(struct bytecode *bc, int *length) {
  int len;

  assert(bc);

  char *buffer = malloc(1024 * 1024);
  assert(buffer);

  char *iter = buffer;

  /* Literal count */
  iter = appendBytes(iter, (char *)&bc->literalsLen, sizeof(bc->literalsLen));

  /* Literals */
  for (int i = 0; i < bc->literalsLen; i++) {
    struct object *literal = bc->literals[i];
    iter = appendBytes(iter, (char *)&literal->type, sizeof(literal->type));

    switch (literal->type) {
    case OBJECT_string:
      /* Length */
      len = strlen(literal->string);
      iter = appendBytes(iter, (char *)&len, sizeof(len));

      /* Contents */
      iter = appendBytes(iter, literal->string, len);
      break;
    }
  }

  /* Function count */
  iter = appendBytes(iter, (char *)&bc->fnsLen, sizeof(bc->fnsLen));

  /* Functions */
  for (int i = 0; i < bc->fnsLen; i++) {
    struct fn *fn = bc->fns[i];

    /* Name length */
    len = strlen(fn->name);
    iter = appendBytes(iter, (char *)&len, sizeof(len));

    /* Name */
    iter = appendBytes(iter, fn->name, len);

    /* Body size */
    iter = appendBytes(iter, (char *)&fn->bodyLen, sizeof(fn->bodyLen));

    /* Body */
    for (int j = 0; j < fn->bodyLen; j++) {
      struct inst inst = fn->body[j];
      iter = appendBytes(iter, (char *)&inst, sizeof(inst));
    }
  }

  /* Done */
  *length = iter - buffer;
  return buffer;
}

struct bytecode *deserializeBytecode(char *s) {
  int len;
  char *buf;

  assert(s);

  struct bytecode *bc = makeBytecode();

  int literalCount = *(int *)s;
  s += sizeof(literalCount);

  for (int i = 0; i < literalCount; i++) {
    enum objectType type = *(enum objectType *)s;
    s += sizeof(type);

    switch (type) {
    case OBJECT_string:
      /* Length */
      len = *(int *)s;
      s += sizeof(len);

      /* Contents */
      buf = malloc(len + 1);
      strncpy(buf, s, len);
      s += len;

      appendLiteral(bc, makeObjectString(buf));
      break;
    }
  }

  int functionCount = *(int *)s;
  s += sizeof(functionCount);

  for (int i = 0; i < functionCount; i++) {
    /* Name length */
    len = *(int *)s;
    s += sizeof(len);

    /* Name */
    buf = malloc(len + 1);
    strncpy(buf, s, len);
    s += len;

    struct fn *fn = makeFn(buf);
    assert(fn);

    /* Body size */
    len = *(int *)s;
    s += sizeof(len);

    /* Body */
    for (int i = 0; i < len; i++) {
      struct inst inst = *(struct inst *)s;
      s += sizeof(inst);
      appendInst(fn, inst);
    }

    appendFn(bc, fn);
  }

  /* Done */
  return bc;
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
