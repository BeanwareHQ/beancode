#include "lexer.h"

#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

Lexer lexer_new(const char* content, size_t content_len) {
    return (Lexer){
        .content = content,
        .content_len = content_len,
        .nlines = 1,
        .bol = 0,
        .pos = 0,
    };
}

bool is_whitespace(char ch) {
    return (ch == ' ' || ch == '\r' || ch == '\t' || ch == '\n');
}

bool is_operator_start(char ch) {
    return (ch == '=' || ch == '>' || ch == '<' || ch == '+' || ch == '-' ||
            ch == '/' || ch == '%' || ch == '*');
}

bool is_separator(char ch) {
    return (ch == '{' || ch == '[' || ch == '(' || ch == ')' || ch == ']' ||
            ch == '}' || ch == '.' || ch == ',');
}

bool is_keyword(const char* str) {
    /* Keywords:
     *
     * DECLARE IF ELSE ENDIF THEN CASE OF FOR NEXT TO REPEAT UNTIL
     * WHILE ENDWHILE FUNCTION ENDFUNCTION RETURN OUTPUT INPUT TRUE
     * FALSE BREAK CONTINUE
     */

    char s[128];
    strcpy(s, str);

    for (size_t i = 0; s[i] != '\0'; i++) {
        s[i] = toupper(s[i]);
    }

    // haha strcmp
    return (!strcmp(s, "DECLARE") || !strcmp(s, "IF") || !strcmp(s, "ELSE") ||
            !strcmp(s, "ENDIF") || !strcmp(s, "THEN") || !strcmp(s, "CASE") ||
            !strcmp(s, "OF") || !strcmp(s, "FOR") || !strcmp(s, "NEXT") ||
            !strcmp(s, "TO") || !strcmp(s, "REPEAT") || !strcmp(s, "UNTIL") ||
            !strcmp(s, "WHILE") || !strcmp(s, "ENDWHILE") ||
            !strcmp(s, "FUNCTION") || !strcmp(s, "ENDFUNCTION") ||
            !strcmp(s, "RETURN") || !strcmp(s, "OUTPUT") ||
            !strcmp(s, "INPUT") || !strcmp(s, "TRUE") || !strcmp(s, "FALSE") ||
            !strcmp(s, "CONTINUE") || !strcmp(s, "BREAK"));
}

bool is_numeral(const char* s) {
    for (size_t i = 0; s[i] != '\0'; i++) {
        if (!isdigit(s[i]) && s[i] != '_' && s[i] != '.')
            return false;
    }
    return true;
}

void lexer_trim_comment(Lexer* lx);

void lexer_trim_left(Lexer* lx) {
    char currch;
    while (is_whitespace((currch = lx->content[lx->pos]))) {
        lx->pos++;

        if (currch == '\n') {
            lx->nlines++;
            lx->bol = lx->pos;
        }
    }

    lexer_trim_comment(lx);
}

void lexer_trim_comment(Lexer* lx) {
    char currch = lx->content[lx->pos];
    if (currch == ';' && lx->content[lx->pos + 1] == ';') {
        while ((currch = lx->content[lx->pos]) != '\n' &&
               lx->pos < lx->content_len)
            lx->pos++;
        lx->nlines++;
        lx->bol = lx->pos;
    }

    if (is_whitespace(lx->content[lx->pos]))
        lexer_trim_left(lx);
}

size_t lexer_next_operator(Lexer* lx, Token* tok, char** res) {
    char op_buf[5];
    size_t nchars = 1;

    strncpy(op_buf, &lx->content[lx->pos], 4);
    op_buf[4] = '\0'; // just in case

    bool is_fourchar_op =
        !strncmp(op_buf, "**->", 4) || !strncmp(op_buf, "//->", 4) ||
        !strncmp(op_buf, "<-//", 4) || !strncmp(op_buf, "<-**", 4);

    if (is_fourchar_op) {
        nchars = 4;
        goto end;
    }

    bool is_threechar_op =
        (!strncmp(op_buf, "<-+", 3) || !strncmp(op_buf, "<--", 3) ||
         !strncmp(op_buf, "<-*", 3) || !strncmp(op_buf, "<-/", 3) ||
         !strncmp(op_buf, "+->", 3) || !strncmp(op_buf, "-->", 3) ||
         !strncmp(op_buf, "*->", 3) || !strncmp(op_buf, "/->", 3));

    if (is_threechar_op) {
        nchars = 3;
        goto end;
    }

    bool is_doublechar_op =
        (!strncmp(op_buf, "==", 2) || !strncmp(op_buf, ">=", 2) ||
         !strncmp(op_buf, "<=", 2) || !strncmp(op_buf, "<>", 2) ||
         !strncmp(op_buf, ">>", 2) || !strncmp(op_buf, "<<", 2) ||
         !strncmp(op_buf, "->", 2) || !strncmp(op_buf, "<-", 2) ||
         !strncmp(op_buf, "**", 2) || !strncmp(op_buf, "//", 2));

    if (is_doublechar_op) {
        nchars = 2;
        goto end;
    }

    // is_operator_start ensures a valid 1-char-long operator

    nchars = 1;
    goto end;

end:
    *res = malloc(nchars + 1);
    strncpy(*res, op_buf, nchars);
    return nchars;
}

size_t lexer_next_string_literal(Lexer* lx, char** txt) {
    size_t bufsize = 32;
    size_t len = 1;
    char currch;
    char* buf = malloc(32);

    if (buf == NULL) {
        perror("realloc: ");
        exit(EXIT_FAILURE);
    }

    buf[0] = '"';

    lx->pos++;
    while ((currch = lx->content[lx->pos]) != '"' && currch != '\n') {
        if (len + 2 > 32) { // len(text + '"' + \0) >= 32
            buf = realloc(buf, bufsize * 2);

            if (buf == NULL) {
                perror("realloc: ");
                exit(EXIT_FAILURE);
            }
        }

        buf[len++] = currch;
        lx->pos++;
    }
    lx->pos++; // skip past the end quote mark
    len++; // end quote mark exists and the loop didn't read it into the buffer

    buf[len - 1] = '"';
    buf[len] = '\0';

    *txt = buf;

    return len;
}

Token lexer_next(Lexer* lx) {
    lexer_trim_left(lx);

    char* text_buf = calloc(256, 1);
    size_t text_len = 0;
    Token tok = {0};

    if (text_buf == NULL) {
        perror("malloc: ");
        exit(EXIT_FAILURE);
    }

    if (lx->pos >= lx->content_len) {
        tok.kind = TOKEN_EOF;
        tok.row = lx->nlines;
        tok.col = lx->pos - lx->bol;
        return tok;
    }

    // lexing part
    char currch = lx->content[lx->pos];

    // operator
    if (is_operator_start(currch)) {
        char* res = NULL;

        size_t nchars = lexer_next_operator(lx, &tok, &res);
        strncpy(text_buf, res, nchars);
        free(res);

        tok.kind = TOKEN_OPERATOR;

        lx->pos += nchars;

        goto end;
    }

    // separators
    if (is_separator(currch)) {
        text_buf = realloc(text_buf, 2);

        if (text_buf == NULL) {
            perror("realloc: ");
            exit(EXIT_FAILURE);
        }

        tok.kind = TOKEN_SEPERATOR;
        text_buf[0] = currch;
        text_buf[1] = '\0';

        lx->pos++;
        goto end;
    }

    // string literals
    if (currch == '"') {
        free(text_buf);
        text_len = lexer_next_string_literal(lx, &text_buf);
        tok.kind = TOKEN_LITERAL;
        goto end;
    }

    // words
    while (!is_separator((currch = lx->content[lx->pos])) &&
           !is_operator_start(currch) && !is_whitespace(currch) &&
           lx->pos < lx->content_len) {
        text_buf[text_len++] = currch;
        lx->pos++;
    }

    text_buf[text_len] = '\0';

    if (is_numeral(text_buf)) {
        tok.kind = TOKEN_LITERAL;
        goto end;
    }

    if (is_keyword(text_buf)) {
        tok.kind = TOKEN_KEYWORD;
        goto end;
    }

    goto end;

end:
    tok.text_len = text_len;
    tok.text = text_buf;
    tok.row = lx->nlines;
    tok.col = lx->pos - lx->bol;

    return tok;
}

size_t lexer_run(Lexer* lx, Token* res) {
    Token* toks = malloc(sizeof(Token) * 256);
    size_t toks_cap = 255;
    size_t toks_len = 0;

    if (toks == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    do {
        if (toks_len + 1 > toks_cap) {
            toks = realloc(toks, sizeof(Token) * toks_cap * 3);

            if (toks == NULL) {
                perror("realloc");
                exit(EXIT_FAILURE);
            }
        }

        toks[toks_len++] = lexer_next(lx);
    } while (toks[toks_len - 1].kind != TOKEN_EOF);

    for (size_t i = 0; i < toks_len; i++) {
        Token t = toks[i];
        char* tok_kind_txt;

        switch (t.kind) {
            case TOKEN_IDENTIFIER: {
                tok_kind_txt = "ident";
            } break;
            case TOKEN_LITERAL: {
                tok_kind_txt = "literal";
            } break;
            case TOKEN_KEYWORD: {
                tok_kind_txt = "keyword";
            } break;
            case TOKEN_SEPERATOR: {
                tok_kind_txt = "separator";
            } break;
            case TOKEN_OPERATOR: {
                tok_kind_txt = "operator";
            } break;
            case TOKEN_NOT_IMPLEMENTED: {
                tok_kind_txt = "notimplemented";
            } break;
            case TOKEN_EOF: {
                tok_kind_txt = "eof";
            } break;
        }

        printf("Token(%s): `%s` @ row %zu, col %zu\n", tok_kind_txt, t.text,
               t.row, t.col);
    }

    res = toks;
    return toks_len;
}
