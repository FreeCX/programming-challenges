fn main() {
    for index in 1..16 {
        match (index % 3, index % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", index),
        };
    }
}
