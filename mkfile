# Example

bin/example.hc: bin/example.htok bin/hc
	cat bin/example.htok | bin/hc > bin/example.hc

bin/example.htok: example.hue bin/htok
	cat example.hue | bin/htok > bin/example.htok

# Binaries

bin/hc: src/hc.c
	mkdir -p bin
	cc -o $target $prereq

bin/htok: src/htok.c
	mkdir -p bin
	cc -o $target $prereq

clean:
	rm -rf bin
