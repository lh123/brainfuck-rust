# Brainfuck
使用 Rust 实现的 Brainfuck 语言解释器，支持使用 JIT 加速执行速度

# Usage
```
Usage: brainfuck-rust.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  

Options:
  -o, --optimize     Optimize code
  -i, --interpreter  Interpreter mode
  -h, --help         Print help
  -V, --version      Print versio
```

### Example
1. 打印 Mendelbrot
```sh
cargo run ./bf/mendelbrot.bf
```

2. 打印 HelloWorld
```sh
cargo run ./bf/test.bf
```