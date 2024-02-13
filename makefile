CC = clang
CFLAGS = -Wall -Wpedantic -g
files := src/main.c src/lexer.c

bean: $(files)
	$(CC) $(CFLAGS) -o bean $(files)

clean:
	rm -f *.o bean
