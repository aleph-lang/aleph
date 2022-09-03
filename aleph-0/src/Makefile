RESULT = al0c
CONTRIB= ../../../aleph-contrib/src

default:
	ocamlc -o $(RESULT) -I $(CONTRIB)/utils dynlink.cma str.cma syntax.cmo filter.cmo main.ml

utils:
	if [ -s utils/filter.cmo ]; then echo "utils already make"; else make -C utils; fi

clean:
	rm -f *.cmo *.cmi