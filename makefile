CC = clang
CFLAGS = -Wall -Wpedantic -g
files := src/parser.c src/main.c src/lexer.c

bean: $(files)
	make -C submodules/beanutils
	$(CC) $(CFLAGS) -o bean $(files) submodules/beanutils/beanutils.o
	rm -f submodules/beanutils/beanutils.o

clean:
	rm -f *.o bean
