#ifndef HUE_LEXER_H
#define HUE_LEXER_H

enum tokenType {
  TOKEN_fn,
  TOKEN_symbol,
  TOKEN_colon,
  TOKEN_end,
  TOKEN_print,
  TOKEN_string
};

struct token {
  enum tokenType type;
  char *value;
};

struct token *nextToken(char **base);
struct token *eatToken(enum tokenType type, char **base);
struct token *tryEatingToken(enum tokenType type, char **base);

#endif
