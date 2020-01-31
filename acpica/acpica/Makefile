.PHONY: build clean

CC=clang
CFLAGS=-nostdlib -m64 -Wall -fno-stack-protector -nodefaultlibs

FILES=$(wildcard src/*.c)
OBJS=$(patsubst %.c, %.o, $(FILES))

build: acpica.a

acpica.o: $(OBJS)
	$(LD) -r $^ -o $@

acpica.a: $(OBJS)
	$(AR) rcs $@ $^

clean:
	rm -f $(OBJS) acpica.a

%.o: %.c
	$(CC) $(CFLAGS) -c $^ -o $@
