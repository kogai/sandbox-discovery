NAME := sandbox-discovery
TARGET := thumbv7m-none-eabi

all: target/$(TARGET)/debug/main

target/$(TARGET)/debug/main:
	cargo build --target $(TARGET)

.PHONY: elfhd
elfhd:
	cargo readobj --bin $(NAME) -- -file-headers
