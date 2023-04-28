# Alephc

Launching Alephc:

to build and run the webservices with default features:\
```cargo run```

to build and run the webservice  with all features:\
```cargo run --all-features```

to build and run the webservice with specific features\
```cargo run --features python_gen,java_gen```

# Simple example with parsing python and gen Ocaml

```cargo run --all-features -- -i py -o ocaml < test/dataset/python/testInt.py```

or

```cargo run --all-features -- -i python -o ocaml < test/dataset/python/testInt.py```
