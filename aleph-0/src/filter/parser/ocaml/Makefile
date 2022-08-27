RESULT = al0c
OCAMLLDFLAGS=-warn-error -31

default: $(RESULT)
$(RESULT): debug-code
	rm -rf $(RESULT)

SOURCES = ../../../utils/syntax.ml ../../../utils/filter.ml parser.mly lexer.mll ocamlParser.ml

TRASH = *.cmo  *.cmi

include OCamlMakefile