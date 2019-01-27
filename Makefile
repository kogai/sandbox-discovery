SRC := $(wildcard ./**/*.rs)
NAME := app
TARGET := thumbv7m-none-eabi

all: target/$(TARGET)/debug/$(NAME)
release: target/$(TARGET)/release/$(NAME)
hello: target/$(TARGET)/debug/examples/hello

target/$(TARGET)/debug/$(NAME): $(SRC)
	cargo build --target $(TARGET)

target/$(TARGET)/release/$(NAME): $(SRC)
	cargo build --target $(TARGET) --release

target/$(TARGET)/debug/examples/hello: $(SRC)
	cargo build --example hello

.PHONY: exec_hello
exec_hello: target/$(TARGET)/debug/examples/hello
	qemu-system-arm \
		-cpu cortex-m3 \
		-machine lm3s6965evb \
		-nographic \
		-semihosting-config enable=on,target=native \
		-kernel target/$(TARGET)/debug/examples/hello

.PHONY: elfhd
elfhd:
	cargo readobj --bin $(NAME) -- -file-headers

.PHONY: size
size:
	cargo size --bin $(NAME) --release -- -A

.PHONY: objdump
objdump:
	cargo objdump --bin $(NAME) --release -- -disassemble -no-show-raw-insn -print-imm-hex

.PHONY: install
install:
	rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
	cargo install cargo-generate
