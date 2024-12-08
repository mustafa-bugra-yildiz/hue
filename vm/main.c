#include "bytecode.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
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

  return 0;
}
