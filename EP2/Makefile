default: main.rs
	rustc -O main.rs -o exec 

generator: ./test/gen.cpp
	g++ ./test/gen.cpp -o ./test/gen -O2

test: default generator
	bash ./test/test.sh

bench: default
	bash ./test/bench.sh

clean: 
	rm -rf ./output/* ./exec ./test/gen
