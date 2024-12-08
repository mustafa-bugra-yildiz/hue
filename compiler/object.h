#ifndef HUE_OBJECT_H
#define HUE_OBJECT_H

enum objectType {
  OBJECT_string,
};

struct object {
  enum objectType type;
  union {
    char *string;
  };
};

struct object *makeObjectString(char *value);

#endif
