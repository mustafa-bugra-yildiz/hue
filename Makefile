CFLAGS += -std=c11 -Wall -Wextra -Werror

COMPILER_SRC = $(wildcard compiler/*.c)
COMPILER_OBJ = $(COMPILER_SRC:.c=.o)

VM_SRC = $(wildcard vm/*.c)
VM_OBJ = $(VM_SRC:.c=.o)

.PHONY: test
test: bin/compiler bin/vm
	bin/compiler example.hue

.PHONY: clean
clean:
	rm $(COMPILER_OBJ) $(VM_OBJ)
	rm -rf bin

bin/compiler: $(COMPILER_OBJ)
	mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $^

bin/vm: $(VM_OBJ)
	mkdir -p bin
	$(CC) $(CFLAGS) -o $@ $^
