# programming-challenges

![challenges](https://i.imgur.com/2n1IBkC.png)

## How to compile
To build a project, you need to generate a `Cargo.toml` file
```bash
$ python build.py
```

Now you may use `cargo` command for compiling!

For example, let's compile `43-morse.rs` code:
```bash
$ cargo build --bin 43-morse
```

And run for execution
```bash
$ cargo run --bin 43-morse
     Running `target/debug/43-morse`
ПРИВЕТ МЕДВЕДЬ<КОНЕЦ СВЯЗИ>
```

## App list
- [01 - Head/Tails](src/01-head-tails.rs)
- [02 - Temperature Converter](src/02-temperature.rs)
- [03 - Calculate Age in Seconds](src/03-age.rs)
- [04 - Encrypt/Decrypt Algorithm](src/04-encrypt-decrypt.rs)
- [05 - FizzBuzz](src/05-fizzbuzz.rs)
- [06 - Rock Paper Scissors](src/06-rock-paper-scissors.rs)
- [09 - Pseudorandom Sentence Generator](src/09-pseudo.rs)
- [10 - Password Generator](src/10-password.rs)
- [14 - Collatz Conjecture](src/14-collatz.rs)
- [15 - Reverse a String](src/15-reverse.rs)
- [17 - Count Words in a String](src/17-words.rs)
- [20 - BMI Calculator](src/20-bmi.rs)
- [24 - Decimal/Binary Converter](https://github.com/FreeCX/rust-by-example/blob/master/src/digit/main.rs)
- [27 - Fibonnaci Sequence](src/27-fib-seq.rs)
- [35 - Currency Converter](src/35-currency-converter.rs)
- [41 - ASCII Digital Clock](src/41-ascii-digital-clock.rs)
- [43 - Text/Morse Translator](src/43-morse.rs), [blog post](https://freecx.github.io/blog/2016/09/07/sound-generator-for-morse)
- [55 - RPN Calculator](https://github.com/FreeCX/rust-by-example/blob/master/src/rpn/main.rs)
- [56 - Mandelbrot Set](src/56-mandlebrot-set.rs)
- [68 - Graphical Analog Clock](src/68-analog-clock.rs)
- [74 - Quine](src/74-quine.rs)
- [76 - Brainfuck Interpreter](src/76-brainfuck.rs), blog post [#1](https://freecx.github.io/blog/2021/12/29/bf-interp) and [#2](https://freecx.github.io/blog/2022/04/11/bf-llvm)
- [96 - Calculate Dot-&-Cross of 2 Vectors](src/96-vector.rs)
