CC= ocamlc
RESULT= al0c

default: lexer parser
	$(CC) -c parser.mli lexer.ml parser.ml $(RESULT).ml
	$(CC) -o $(RESULT) lexer.cmo parser.cmo $(RESULT).cmo

clean:
	rm -rf lexer.ml parser.ml *.cmi *.cmo *.mli $(RESULT)

lexer:
	ocamllex lexer.mll

parser:
	ocamlyacc parser.mly