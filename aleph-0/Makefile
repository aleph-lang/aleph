RESULT= al0c

default: 
	make -C src
	mv src/$(RESULT) $(RESULT)

clean:
	make -C src clean

cleanest: clean
	rm -f $(RESULT)