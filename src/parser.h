#pragma once

#include "lexer.h"

typedef struct {
    const Token* toks;
    const size_t ntoks;
    size_t pos;
} Parser;

typedef enum {
    NODE_EXPR,

    // statements
    NODE_STMT_ASSIGN,
    NODE_STMT_OUTPUT,
    NODE_STMT_INPUT,
    NODE_STMT_TOPLEVEL,

    // other
    NODE_NOT_IMPLEMENTED,
} AstNodeKind;

typedef struct {
    AstNodeKind kind;
    void* data; // ptr to AstNode_x
} AstNode;

typedef struct {
    AstNode* nodes;
} AstNode_Toplevel;

typedef struct {
    Token ident;
    AstNode expr;
} AstNode_Assign;

typedef struct {
    AstNode expr;
} AstNode_Output;

typedef struct {
    AstNode expr;
} AstNode_Input;

typedef struct {

} AstNode_Expression;
