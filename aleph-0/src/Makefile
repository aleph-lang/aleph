RESULT = al0c

default:
	ocamlc -o $(RESULT) -I utils dynlink.cma str.cma syntax.cmo filter.cmo main.ml

utils:
	if [ -s utils/filter.cmo ]; then echo "utils already make"; else make -C utils; fi

clean:
	rm -f *.cmo *.cmi