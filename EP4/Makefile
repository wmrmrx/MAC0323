default: 
	rustc --edition=2021 -O main.rs -o exec

test: default
	bash ./tests/validate.sh

clean:
	rm -f exec
