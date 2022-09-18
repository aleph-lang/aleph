RESULT = alephc
CONTRIB= ../../aleph-contrib/src

default:
	ocamlc -o $(RESULT) -I $(CONTRIB)/utils dynlink.cma str.cma syntax.cmo filter.cmo classUtils.cmo main.ml

utils:
	if [ -s $(CONTRIB)/utils/filter.cmo ]; then echo "utils already make"; else make -C $(CONTRIB)/utils; fi

clean:
	rm -f *.cmo *.cmi