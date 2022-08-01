RESULT = al0c
OCAMLLDFLAGS=-warn-error -31

default: $(RESULT)
$(RESULT): debug-code

SOURCES = frontend/type.ml frontend/id.ml frontend/m.ml \
frontend/syntax.ml frontend/parser.mly frontend/lexer.mll \
backend/emit.ml main.ml

include OCamlMakefile
