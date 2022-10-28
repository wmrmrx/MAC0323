default: main.rs 
	rustc -O main.rs -o exec

test: exec
	bash tests/test.sh

clean: 
	rm -f exec
