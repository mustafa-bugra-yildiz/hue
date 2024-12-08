#include "object.h"

#include <assert.h>
#include <stdlib.h>

struct object *makeObjectString(char *value) {
  assert(value);

  struct object *ptr = malloc(sizeof(*ptr));
  assert(ptr);

  ptr->type = OBJECT_string;
  ptr->string = value;

  return ptr;
}
