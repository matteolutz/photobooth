# CC = gcc
# CC_FLAGS = -F /Library/Frameworks -framework EDSDK

CC = clang
CC_FLAGS = -F /Library/Frameworks -framework EDSDK -Wl,-rpath,/Library/Frameworks

main: main.c
	$(CC) $(CC_FLAGS) -o main main.c

run: main
	./main

clean:
	rm main

.PHONY: clean run
