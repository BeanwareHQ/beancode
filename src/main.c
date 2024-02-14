#include "lexer.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char* argv[argc]) {
    FILE* file;
    size_t buf_size = 2048;
    char* buf = malloc(buf_size);
    size_t buf_len = 0;

    if (argc == 1) {
        fprintf(stderr, "not enough args\n");
        exit(EXIT_FAILURE);
    }

    if (buf == NULL) {
        perror("realloc");
        exit(EXIT_FAILURE);
    }

    file = fopen(argv[1], "r");

    if (file == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    char currch;
    while ((currch = fgetc(file)) != EOF) {
        if (buf_len + 2 > 2048) {
            buf_size *= 3;
            buf = realloc(buf, buf_size);

            if (buf == NULL) {
                perror("realloc: ");
                exit(EXIT_FAILURE);
            }
        }

        buf[buf_len++] = currch;
    }

    fclose(file);

    buf[buf_len] = '\0';

    Lexer lexer = lexer_new(buf, strlen(buf));
    Token* tokens;
    size_t tokens_len = lexer_run(&lexer, tokens);
}
