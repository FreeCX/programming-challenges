# programming-challenges

![challenges](https://i.imgur.com/2n1IBkC.png)

## How to compile
For first time you need to generate `Cargo.toml` file.
```bash
$ python build.py
```

Now you may use `cargo` for compile binaries.

For example let compile `43-morse.rs`:
```bash
$ cargo build --bin 43-morse
```

And run binary.
```bash
$ cargo run --bin 43-morse
     Running `target/debug/43-morse`
ПРИВЕТ МЕДВЕДЬ<КОНЕЦ СВЯЗИ>
```

## Extra
* [24 - Decimal/Binary Converter](https://github.com/FreeCX/rust-by-example/blob/master/src/digit/main.rs)
* [43 - Text/Morse Translator](https://freecx.github.io/blog/2016/09/07/sound-generator-for-morse) (RUS, article)
* [55 - RPN Calculator](https://github.com/FreeCX/rust-by-example/blob/master/src/rpn/main.rs)
* [76 - Brainfuck Interpreter](https://github.com/FreeCX/rust-by-example/blob/master/src/brainfuck/main.rs) (incomplete)