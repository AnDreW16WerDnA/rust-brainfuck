# Rust Brainfuck

This program allows you to run brainfuck code using the programming language rust.

**PS. I made this as an attempt at learning rust**

## Building the binary file

For development
```bash
cargo build
```

For release
```bash
cargo build release
```

## Installing it

First of all you need to build the realease version, then you copy the binary to `/usr/bin/bf`:

```bash
sudo cp target/release/brainfuck /usr/bin/bf
```

