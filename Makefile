.PHONY: all
all:
	cargo run example.hue > example.s
	cat example.s
