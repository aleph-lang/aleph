RESULT = al0c
OCAMLLDFLAGS=-warn-error -31

default: $(RESULT)

$(RESULT): debug-code
	ocamlc -o $(RESULT) -I utils dynlink.cma str.cma syntax.cmo filter.cmo main.ml

SOURCES = utils/syntax.ml utils/filter.ml

TRASH = main.cmo main.cmi filter/*.cmi filter/*.cmo filter/*/*.cmi filter/*/*.cmo  filter/*/*/*.cmi filter/*/*/*.cmo

include OCamlMakefile