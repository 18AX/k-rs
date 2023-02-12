CC?=gcc
CFLAGS=-ffreestanding -fno-stack-protector -fno-pic -fpie -mno-80387 -mno-mmx -mno-3dnow -mno-sse -mno-sse2 -mno-red-zone
LFLAGS=-nostdlib -T k-rs.ld -Wl,-static,-no-pie,--no-dynamic-linker,-ztext -static-pie -nostdlib -z max-page-size=0x1000

TARGET=k-rs

LIBK=target/x86_64-unknown-none/release/libk.a
ISO=k-rs.iso

all: $(TARGET)

.PHONY: $(TARGET) all clean

crt0.o: crt0.S
	gcc $(CFLAGS) -c crt0.S -o crt0.o

$(TARGET): crt0.o
	cargo build --release
	$(CC) $(LFLAGS) crt0.o $(LIBK) -o k-rs

iso: $(TARGET)
	./makeiso.sh

clean:
	$(RM) *.o $(TARGET) $(ISO)
	$(RM) -rf iso
	cargo clean