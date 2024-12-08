#ifndef HUE_BYTECODE_H
#define HUE_BYTECODE_H

#include "object.h"

enum instType {
  INST_loadLit,
  INST_print,
  INST_halt,
};

struct inst {
  enum instType type;
  int arg;
};

struct fn {
  char *name;
  struct inst *body;
  int bodyLen, bodyCap;
};

struct fn *makeFn(char *name);
void appendInst(struct fn *fn, struct inst inst);

struct bytecode {
  struct object **literals;
  int literalsLen, literalsCap;

  struct fn **fns;
  int fnsLen, fnsCap;
};

struct bytecode *makeBytecode();
char *serializeBytecode(struct bytecode *bc, int *length);
struct bytecode *deserializeBytecode(char *code);

void printBytecode(struct bytecode *bc);
int appendLiteral(struct bytecode *bc, struct object *object);
void appendFn(struct bytecode *bc, struct fn *fn);

#endif
