# Hack VM Translator

A VM-to-assembly translator written in Rust, built as part of the [Nand to Tetris](https://www.nand2tetris.org/) course (Project 7). It translates programs written in the Hack VM language into Hack assembly (`.asm`) files.

## Overview

The Hack VM is a stack-based virtual machine. This translator reads `.vm` source files and emits equivalent Hack assembly that can then be assembled into binary machine code for the Hack computer.

The translation pipeline is:

```
.vm file  →  Lexer  →  Parser  →  Converter  →  .asm file
```

## Project Structure

```
vm_translator/
├── src/
│   ├── main.rs        # Entry point, CLI argument handling, I/O
│   ├── lexer.rs       # Tokenizer — reads VM source line by line
│   ├── command.rs     # Command and Segment type definitions + parsing
│   └── converter.rs   # VM command → Hack assembly translation
├── test_programs/
│   ├── SimpleAdd.vm
│   ├── BasicTest.vm
│   ├── StackTest.vm
│   ├── StaticTest.vm
│   └── PointerTest.vm
├── Cargo.toml
└── Cargo.lock
```

## Building

Requires [Rust](https://www.rust-lang.org/tools/install) and Cargo.

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <path/to/file.vm>
# or, with the release binary:
./target/release/vm_translator <path/to/file.vm>
```

The output `.asm` file is written to the same directory as the input file with the same base name. For example:

```bash
cargo run -- test_programs/SimpleAdd.vm
# produces: test_programs/SimpleAdd.asm
```

## Supported VM Commands

### Memory Access

| Command | Description |
|---|---|
| `push <segment> <index>` | Push a value from a memory segment onto the stack |
| `pop <segment> <index>` | Pop the top of the stack into a memory segment |

### Arithmetic

| Command | Description |
|---|---|
| `add` | Add top two stack values |
| `sub` | Subtract top from second-to-top |
| `neg` | Negate the top stack value |

### Logical / Bitwise

| Command | Description |
|---|---|
| `and` | Bitwise AND of top two stack values |
| `or` | Bitwise OR of top two stack values |
| `not` | Bitwise NOT of the top stack value |

### Comparison

Comparison commands push `-1` (true) or `0` (false) onto the stack.

| Command | Description |
|---|---|
| `eq` | Equal |
| `gt` | Greater than |
| `lt` | Less than |

## Supported Memory Segments

| Segment | Description |
|---|---|
| `constant` | Literal integer constants — loaded directly as immediate values |
| `local` | Local variables for the current function (base: `LCL`) |
| `argument` | Arguments passed to the current function (base: `ARG`) |
| `this` | Fields of the current object (base: `THIS`) |
| `that` | Array elements (base: `THAT`) |
| `temp` | Shared temporary variables, indices 0–7 (base address: 5) |
| `pointer` | Index 0 sets/gets `THIS`, index 1 sets/gets `THAT` |
| `static` | File-level static variables, referenced as `<filename>.<index>` |

## Running Tests

Unit tests for command parsing live in `src/command.rs`.

```bash
cargo test
```

## Test Programs

The `test_programs/` directory contains sample `.vm` files from the Nand to Tetris course:

| File | What it tests |
|---|---|
| `SimpleAdd.vm` | Basic push and arithmetic |
| `StackTest.vm` | All arithmetic, logical, and comparison operations |
| `BasicTest.vm` | Push/pop across local, argument, this, that, and temp segments |
| `StaticTest.vm` | Static memory segment |
| `PointerTest.vm` | Pointer segment and indirect memory access via this/that |
