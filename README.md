# WS

Launching Aleph-WS:

to build and run the webservices with default features:\
```cargo run```

to build and run the webservice  with all features:\
```cargo run --all-features```

to build and run the webservice with specific features\
```cargo run --features python_gen,java_gen```

# Client

Launching Aleph-Client:

```cargo run --bin aleph-cli -- -i json -o ale < fileToBeRead```

# Alephc (without WS)

Launching Alephc:

```cargo run --bin alephc -- -i json -o ale < fileToBeRead```

