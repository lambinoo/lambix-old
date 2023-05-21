.PHONY: build rebuild clean build-iso symbols

# name
KERNEL_NAME=lambix
TARGET_TRIPLE=x86_64-unknown-$(KERNEL_NAME)

# binaries
GRUB_MKRESCUE = grub2-mkrescue
QEMU=qemu-system-x86_64
GDB=gdb

# config
ifneq ($(PROFILE),release)
	PROFILE=debug
endif

ifndef RUSTFLAGS
	RUSTFLAGS="-A dead_code"
endif

# path
BUILD_DIR = target
LINKER_SCRIPT = linker.ld
KERNEL = target/$(TARGET_TRIPLE)/$(PROFILE)/$(KERNEL_NAME)
KERNEL_ISO = $(BUILD_DIR)/isodir/boot/$(KERNEL_NAME)

# debug
QEMU_FLAGS=-cdrom "$(BUILD_DIR)/$(KERNEL_NAME).iso" --enable-kvm -no-reboot -no-shutdown -m 4G -smp 4

ifeq ($(PROFILE),release)
    CARGO_FLAGS = --release
endif

build-iso: build symbols
	mkdir -p $(BUILD_DIR)/isodir/boot/grub
	cp $(KERNEL) $(KERNEL_ISO)
	cp grub.cfg $(BUILD_DIR)/isodir/boot/grub
	$(GRUB_MKRESCUE) -o $(BUILD_DIR)/$(KERNEL_NAME).iso $(BUILD_DIR)/isodir 2> $(BUILD_DIR)/grub_mkrescue.log 

# Build the kernel
build:
	RUSTFLAGS=$(RUSTFLAGS) cargo build --target=$(shell pwd)/$(TARGET_TRIPLE).json $(CARGO_FLAGS)

check:
	RUSTFLAGS=$(RUSTFLAGS) cargo check

symbols:
	nm $(KERNEL) | awk '{print $$1 " " $$3}' > $(BUILD_DIR)/symbols 

# Clean and assembler object files
clean:
	cargo clean
	rm -f $(BUILD_DIR)/$(KERNEL_NAME).iso
	rm -f $(BUILD_DIR)/grub_mkrescue.log
	rm -f $(BUILD_DIR)/symbols


# Rebuild the kernel from scratch
rebuild: clean build

debug:
	$(GDB) \
		-ex "target remote | $(QEMU) $(QEMU_FLAGS) -S -gdb stdio -monitor pty -serial file:lambix.log" \
		-ex "symbol-file $(KERNEL_ISO)" \

run: build-iso
	$(QEMU) $(QEMU_FLAGS) -serial stdio -vga std

