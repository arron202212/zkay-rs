SHELL=/bin/bash

all: grammar

grammar: clean
	java -jar ./antlr4-4.8-2-SNAPSHOT-complete.jar -visitor -Dlanguage=Rust -o generated Solidity.g4

test: grammar
	bash -c "cd ..; source ./activate.sh; python -m unittest solidity_parser/*.py"

clean:
	rm -rf generated
