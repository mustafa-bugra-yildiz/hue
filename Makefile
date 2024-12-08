CFLAGS += -std=c11 -Wall -Wextra -Werror -Ilib
LIBS = -Lbin -llib

LIB_SRC = $(wildcard lib/*.c)
LIB_OBJ = $(LIB_SRC:.c=.o)

COMPILER_SRC = $(wildcard compiler/*.c)
COMPILER_OBJ = $(COMPILER_SRC:.c=.o)

VM_SRC = $(wildcard vm/*.c)
VM_OBJ = $(VM_SRC:.c=.o)

.PHONY: test
test: bin/liblib.a bin/compiler bin/vm
	bin/compiler -i example.hue -o example.huec
	bin/vm -i example.huec

.PHONY: clean
clean:
	rm $(COMPILER_OBJ) $(VM_OBJ) $(LIB_OBJ)
	rm -rf bin

bin/compiler: $(COMPILER_OBJ)
	mkdir -p bin
	$(CC) $(CFLAGS) $(LIBS) -o $@ $^

bin/vm: $(VM_OBJ)
	mkdir -p bin
	$(CC) $(CFLAGS) $(LIBS) -o $@ $^

bin/liblib.a: $(LIB_OBJ)
	mkdir -p bin
	ar r $@ $^
