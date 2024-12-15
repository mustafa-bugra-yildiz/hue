.PHONY: all
all:
	cargo run example.hue > example.s
	cat example.s
	as example.s
	rm example.o
