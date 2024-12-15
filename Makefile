.PHONY: all
all:
	cargo run example.hue | tee example.s
	as example.s
	rm example.o
