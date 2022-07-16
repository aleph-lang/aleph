RESULT= al0c

default: mdir
	make -C src
	mv src/$(RESULT) $(RESULT)

clean:
	make -C src clean

cleanest: clean
	rm -rf out $(RESULT)

mdir:
	mkdir -p out