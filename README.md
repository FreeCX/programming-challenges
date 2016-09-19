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
