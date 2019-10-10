.PHONY: build rebuild clean build-iso symbols

# name
KERNEL_NAME=lambix
TARGET_TRIPLE=x86_64-unknown-$(KERNEL_NAME)

# binaries
GRUB_MKRESCUE = grub-mkrescue
QEMU=qemu-system-x86_64
GDB=rust-gdb

# config
ifneq ($(PROFILE),release)
    PROFILE=debug
endif

# path
BUILD_DIR = build
LINKER_SCRIPT = linker.ld
KERNEL = target/$(TARGET_TRIPLE)/$(PROFILE)/$(KERNEL_NAME)

# debug
QEMU_FLAGS=-vga std -cdrom "$(BUILD_DIR)/$(KERNEL_NAME).iso" --enable-kvm -no-reboot -no-shutdown

ifeq ($(PROFILE),release)
    CARGO_FLAGS = --release
endif

build-iso: build
	mkdir -p $(BUILD_DIR)/isodir/boot/grub
	cp $(KERNEL) $(BUILD_DIR)/isodir/boot/$(KERNEL_NAME)
	cp $(BUILD_DIR)/grub.cfg $(BUILD_DIR)/isodir/boot/grub
	$(GRUB_MKRESCUE) -o $(BUILD_DIR)/$(KERNEL_NAME).iso $(BUILD_DIR)/isodir 2> $(BUILD_DIR)/grub_mkrescue.log 

# Build the kernel
build:
	cargo xbuild --target=$(shell pwd)/$(TARGET_TRIPLE).json $(CARGO_FLAGS)

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
	$(GDB) $(KERNEL) -ex "target remote | $(QEMU) $(QEMU_FLAGS) -S -gdb stdio"

run:
	$(QEMU) $(QEMU_FLAGS) -monitor stdio -S

