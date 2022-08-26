RESULT = al0c
OCAMLLDFLAGS=-warn-error -31

default: $(RESULT) filterParseAl0 filterParseOcaml filterIdent filterIdent2 filterAl0
$(RESULT): debug-code
	ocamlc -o $(RESULT) -I utils -I filter/parser/al0 dynlink.cma str.cma \
	 	filter.cmo id.cmo lexer.cmo m.cmo type.cmo syntax.cmo parser.cmo main.ml

SOURCES = utils/type.ml utils/id.ml utils/m.ml utils/syntax.ml \
filter/parser/al0/parser.mly filter/parser/al0/lexer.mll utils/filter.ml filter/gen/ocaml/ocaml.ml

TRASH = main.cmo main.cmi filter/*.cmi filter/*.cmo filter/*/*.cmi filter/*/*.cmo  filter/*/*/*.cmi filter/*/*/*.cmo

include OCamlMakefile

filterParseAl0:
	ocamlc -I utils -I filter/parser/al0 -c filter/parser/al0/al0Parser.ml

filterParseOcaml:
	ocamlc -I utils -I filter/parser/al0 -c filter/parser/ocaml/ocamlParser.ml

filterAl0:
	ocamlc -I utils -c filter/gen/al0/al0.ml

filterIdent:
	ocamlc -I utils -c filter/transform/ident.ml

filterIdent2:
	ocamlc -I utils -c filter/transform/ident2.ml