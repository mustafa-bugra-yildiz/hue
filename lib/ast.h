#ifndef HUE_AST_H
#define HUE_AST_H

/* Expressions */

struct exprString {
  char *value;
};

enum exprType {
  EXPR_string,
};

struct expr {
  enum exprType type;
  union {
    struct exprString *string;
  };
};

/* Statements */

struct stmtPrint {
  struct expr *expr;
};

enum stmtType {
  STMT_print,
};

struct stmt {
  enum stmtType type;
  union {
    struct stmtPrint *print;
  };
};

/* Declarations */

struct declFn {
  char *name;
  struct stmt **body;
  int bodyLen, bodyCap;
};

enum declType {
  DECL_fn,
};

struct decl {
  enum declType type;
  union {
    struct declFn *fn;
  };
};

void printDecl(struct decl *decl);

#endif
