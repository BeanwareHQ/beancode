#include <stdlib.h>
#include <string.h>

#include "beanutils.h"
#include "lexer.h"
#include "parser.h"

Parser parser_new(const size_t ntoks, const Token toks[ntoks]) {
    return (Parser){
        .toks = toks,
        .ntoks = ntoks,
        .pos = 0,
    };
}

AstNode parser_handle_keyword(Parser* p, const Token* tok) {
    AstNode res = {0};

    if (!strcmp(tok->text, "output")) {
        // syntax: output <expr>
        if (tok->kind == TOKEN_KEYWORD || tok->kind == TOKEN_OPERATOR ||
            (tok->kind == TOKEN_SEPERATOR && tok->text[0] != '(')) {
            b_log(LOGLEVEL_ERROR,
                  "found invalid token `%s` in an output expression (%zu, %zu)",
                  tok->text, tok->row, tok->col);
            goto end;
        } else if (tok->kind == TOKEN_SEPERATOR && tok->text[0] == '(') {
            b_log(LOGLEVEL_WARN,
                  "compound expressions with `(`'s are not supported yet.");
            goto end;
        }

        AstNode_Output* node = calloc(1, sizeof(AstNode_Output));

        // TODO: parse expression

        res.data = node;
        res.kind = NODE_OUTPUT;

        goto end;
    }

end:
    return res;
}

void parse_next_node(Parser* p, AstNode* newnode) {
    AstNode res = {0};
    const Token* currtok = &p->toks[p->pos];

    switch (currtok->kind) {
        case TOKEN_KEYWORD: {
            res = parser_handle_keyword(p, currtok);
        } break;
        case TOKEN_LITERAL: {
            b_log(LOGLEVEL_WARN, "literal tokens are not implemented");
        } break;
        case TOKEN_IDENTIFIER: {
            b_log(LOGLEVEL_WARN, "identifier tokens are not implemented");
        } break;
        default: {
            b_log(LOGLEVEL_WARN, "found not implemented token at (%zu, %zu)",
                  currtok->row, currtok->col);
        }
    }

    *newnode = res;
}

AstNode_Toplevel* parse(Parser* p, const Token* tokens) {
    AstNode_Toplevel* res = {0};
    AstNode* nodes = calloc(16, sizeof(AstNode));
    size_t nodes_len = 0;

    for (; tokens[p->pos].kind != TOKEN_EOF; p->pos++) {
        AstNode* newnode = NULL;
        parse_next_node(p, newnode);
        if (newnode != NULL)
            nodes[nodes_len++] = *newnode;
    }

    return res;
}
