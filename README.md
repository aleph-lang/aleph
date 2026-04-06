# Aleph Compiler (Alephc)

Alephc is a multi-language compiler/transpiler that can parse and generate code between various programming languages. It supports parsing from Python, JavaScript, JSON, and Aleph language, and can generate code in multiple target languages including Python, Erlang, Elixir, Gleam, and more.

## Features

### Input Languages (Parsers)
- **Aleph Language** ([`ale_parse`](https://github.com/aleph-lang/aleparser)): The native Aleph language
- **Python** ([`python_parse`](https://github.com/aleph-lang/pythonparser)): Parse Python code
- **JavaScript** ([`js_parse`](https://github.com/aleph-lang/jsparser)): Parse JavaScript code
- **JSON** (`json_parse`): Parse JSON structures

### Output Languages (Generators)
- **Aleph Language** ([`ale_gen`](https://github.com/aleph-lang/alegen)): Generate Aleph code
- **Python** ([`python_gen`](https://github.com/aleph-lang/pythongen)): Generate Python code
- **Erlang** ([`erlang_gen`](https://github.com/aleph-lang/erlanggen)): Generate Erlang code
- **Elixir** ([`elixir_gen`](https://github.com/aleph-lang/elixirgen)): Generate Elixir code
- **Gleam** ([`gleam_gen`](https://github.com/aleph-lang/gleamgen)): Generate Gleam code
- **JSON** (`json_gen`): Generate JSON structures

## Installation

### Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Building

```bash
cargo build
```

## Usage

### Basic Command Structure

```bash
cargo run -- [FEATURES] -- -i [INPUT_LANGUAGE] -o [OUTPUT_LANGUAGE] < [INPUT_FILE]
```

### Running with Default Features

The default features include: Aleph parsing/generation, JSON parsing/generation, Erlang generation, and Elixir generation.

```bash
cargo run
```

### Running with All Features

```bash
cargo run --all-features
```

### Running with Specific Features

```bash
cargo run --features python_gen,erlang_gen
```

## Examples

### Parse Python and Generate Ocaml

```bash
cargo run --all-features -- -i py -o ocaml < test/dataset/python/testInt.py
```

Or with full language names:

```bash
cargo run --all-features -- -i python -o ocaml < test/dataset/python/testInt.py
```

### Parse JavaScript and Generate Python

```bash
cargo run --features js_parse,python_gen -- -i js -o python < test/dataset/js/testInt.js
```

### Parse JSON and Generate Erlang

```bash
cargo run --features json_parse,erlang_gen -- -i json -o erlang < test/dataset/json/testInt.json
```

## Supported Language Codes

### Input Language Codes
- `ale` or `aleparse`: Aleph language
- `py` or `python`: Python
- `js` or `javascript`: JavaScript
- `json`: JSON

### Output Language Codes
- `ale` or `alegen`: Aleph language
- `py` or `python`: Python
- `erlang`: Erlang
- `elixir`: Elixir
- `gleam`: Gleam
- `json`: JSON

## Transformers

You can apply code transformations using the `-t` or `--transformer_list` option:

```bash
cargo run --all-features -- -i python -o ocaml -t "transform1,transform2" < input.py
```

## Test Dataset

The project includes a comprehensive test dataset located in `test/dataset/` with examples in:
- Python (`test/dataset/python/`)
- JavaScript (`test/dataset/js/`)
- JSON (`test/dataset/json/`)
- Aleph (`test/dataset/ale/`)
- OCaml (`test/dataset/ocaml/`)

These include various language constructs like:
- Basic types (int, float, string, bool)
- Control structures (if, for, while)
- Functions and classes
- Sorting algorithms
- Mathematical operations

## Project Structure

- `src/main.rs`: Main entry point and CLI interface
- `src/filter/`: Core compilation pipeline
  - `parser/`: Language parsers
  - `gen/`: Code generators
  - `transform/`: Code transformers

## Related Projects

The Aleph compiler ecosystem includes several related projects:

### Core Components
- [Aleph Compiler](https://github.com/aleph-lang/aleph) - Main compiler/transpiler
- [Aleph Syntax Tree](https://github.com/aleph-lang/aleph-syntax-tree) - Core syntax tree structure
- [Beta Reduction](https://github.com/aleph-lang/betareduction) - AST transformation engine

### Experimental & Advanced Tools
- [Aleph Call](https://github.com/aleph-lang/ale_call) - Erlang implementation caller
- [Aleph Ollama](https://github.com/aleph-lang/aleph_ollama) - AI-powered translator
- [Aleph Ollama Erlang](https://github.com/aleph-lang/aleph_ollama_erlang) - Erlang AI translation library

## License

This project is licensed under the terms specified in the LICENSE file.

## Repository

GitHub: https://github.com/aleph-lang/aleph

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Building for Development

```bash
cargo build --release
```

## Using the Compiled Binary

After building, you can use the compiled binary directly:

```bash
./target/release/alephc -i [INPUT_LANGUAGE] -o [OUTPUT_LANGUAGE] < [INPUT_FILE]
```

### Examples with the Binary

```bash
# Parse Python and generate Ocaml using the binary
./target/release/alephc -i python -o ocaml < test/dataset/python/testInt.py

# Parse JavaScript and generate Python using the binary
./target/release/alephc -i js -o python < test/dataset/js/testInt.js
```

The binary supports the same options as `cargo run` but doesn't require specifying features at runtime since features are compiled into the binary during the build process.

## Running Tests

The project includes various test files in the `test/dataset/` directory that you can use to verify functionality:

```bash
# Test Python to Aleph conversion
cargo run --all-features -- -i python -o ale < test/dataset/python/testInt.py

# Test Aleph to JSON conversion  
cargo run --default -- -i ale -o json < test/dataset/ale/testInt.ale
```

## Feature Matrix

| Feature | Description | Default |
|---------|-------------|---------|
| `ale_parse` | Parse Aleph language | ✓ |
| `js_parse` | Parse JavaScript | - |
| `json_parse` | Parse JSON | ✓ |
| `python_parse` | Parse Python | - |
| `ale_gen` | Generate Aleph | ✓ |
| `json_gen` | Generate JSON | ✓ |
| `python_gen` | Generate Python | - |
| `erlang_gen` | Generate Erlang | ✓ |
| `elixir_gen` | Generate Elixir | ✓ |
| `gleam_gen` | Generate Gleam | - |

