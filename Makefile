RESULT= alephc
CONTRIB= ../aleph-contrib/src

default: mdir contrib
	make -C src
	mv src/$(RESULT) $(RESULT)

clean: cleanContrib
	make -C src clean

cleanest: clean
	rm -rf out $(RESULT)

mdir:
	mkdir -p out

cleanContrib:
	$(foreach c,$(CONTRIB),make -C $(c) clean && ) true

contrib:
	$(foreach c,$(CONTRIB),make -C $(c) && ) true