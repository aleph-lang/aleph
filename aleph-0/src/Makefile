RESULT = al0c
OCAMLLDFLAGS=-warn-error -31

default: $(RESULT)
$(RESULT): debug-code

SOURCES = frontend/type.ml frontend/id.ml frontend/m.ml frontend/s.ml \
frontend/syntax.ml frontend/parser.mly frontend/lexer.mll frontend/typing.mli \
frontend/typing.ml frontend/kNormal.mli frontend/kNormal.ml frontend/alpha.mli frontend/alpha.ml \
frontend/reduce/beta.mli frontend/reduce/beta.ml frontend/reduce/assoc.mli frontend/reduce/assoc.ml \
frontend/reduce/inline.mli frontend/reduce/inline.ml frontend/reduce/constFold.mli frontend/reduce/constFold.ml \
frontend/reduce/elim.mli frontend/reduce/elim.ml \
backend/emit.mli backend/emit.ml \
main.mli main.ml

include OCamlMakefile
