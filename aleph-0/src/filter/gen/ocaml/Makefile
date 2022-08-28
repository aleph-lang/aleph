default: utils
	ocamlc -I ../../../utils syntax.cmo filter.cmo -c ocaml.ml

utils:
	if [ -s ../../../utils/filter.cmo ]; then echo "utils already make"; else make -C ../../../utils; fi

clean:
	rm -f *.cmo *.cmi