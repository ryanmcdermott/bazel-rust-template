# bazel-rust-template

This project is a simple starter template for a Rust project that uses the [Bazel build system](https://www.bazel.build/).
There is a main `rust_binary` that runs a simple `rust_library` that adds two numbers together. Additionally, there
is a `rust_test` target created for the adder library.

## Installation

First, ensure that you have [installed Bazel](https://bazel.build/install). Then run the following:

```
git clone https://github.com/ryanmcdermott/bazel-rust-template
```

## Running

### Main

```
bazel run src:main
```

### Tests

```
bazel test src/adder:adder_test
```