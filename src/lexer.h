#pragma once

#include <stddef.h>

#define MAX_TOKENS 2048

typedef enum {
    TOKEN_IDENTIFIER = 0,
    TOKEN_LITERAL = 1,
    TOKEN_KEYWORD = 2,
    TOKEN_SEPERATOR = 3,
    TOKEN_OPERATOR = 4,
    TOKEN_NOT_IMPLEMENTED = 5,
    TOKEN_EOF = 6,
} TokenKind;

typedef struct {
    TokenKind kind;
    size_t row;
    size_t col;
    const char* text;
    size_t text_len;
} Token;

typedef struct {
    const char* content;
    size_t content_len;
    size_t nlines;
    size_t bol;
    size_t pos;
} Lexer;

Lexer lexer_new(const char* file, size_t file_len);
Token lexer_next(Lexer* lx);
size_t lexer_run(Lexer* lx, Token* res);
