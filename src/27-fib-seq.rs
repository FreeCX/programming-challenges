// more info: https://freecx.github.io/blog/2020/02/22/biguint-fib (RUS, article)
fn fib_seq(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let c = b;
        b = a + b;
        a = c;
    }
    a
}

fn main() {
    println!("fib_seq(5) = {}", fib_seq(5));
}
