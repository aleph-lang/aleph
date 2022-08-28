RESULT= al0c
FILTER= src/utils $(dir $(wildcard src/filter/*/*/.))

default: mdir filters
	make -C src
	mv src/$(RESULT) $(RESULT)
	rm -rf src/filter/utils

clean: cleanFilters
	make -C src clean

cleanest: clean
	rm -rf out $(RESULT)

mdir:
	mkdir -p out

cleanFilters:
	$(foreach c,$(FILTER),make -C $(c) clean && ) true

filters:
	$(foreach c,$(FILTER),make -C $(c) && ) true