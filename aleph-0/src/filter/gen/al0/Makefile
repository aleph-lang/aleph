default: utils
	ocamlc -I ../../../utils syntax.cmo filter.cmo -c al0.ml

utils:
	if [ -s ../../../utils/filter.cmo ]; then echo "utils already make"; else make -C ../../../utils; fi

clean:
	rm -f *.cmo *.cmi