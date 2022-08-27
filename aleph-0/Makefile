RESULT= al0c

default: mdir filterParseAl0 filterParseOcaml filterIdent filterIdent2 filterGenAl0 filterGenOcaml
	make -C src
	mv src/$(RESULT) $(RESULT)
	rm -rf src/filter/utils

clean: cleanParseAl0 cleanParseOcaml
	make -C src clean

cleanest: clean
	rm -rf out $(RESULT)

mdir:
	mkdir -p out

cleanParseAl0:
	make -C src/filter/parser/al0 clean

filterParseAl0:
	make -C src/filter/parser/al0

cleanParseOcaml:
	make -C src/filter/parser/ocaml clean

filterParseOcaml:
	make -C src/filter/parser/ocaml

filterIdent:
	ocamlc -I src/utils -c src/filter/transform/ident.ml

filterIdent2:
	ocamlc -I src/utils -c src/filter/transform/ident2.ml

filterGenAl0:
	ocamlc -I src/utils -c src/filter/gen/al0/al0.ml

filterGenOcaml:
	ocamlc -I src/utils -c src/filter/gen/ocaml/ocaml.ml