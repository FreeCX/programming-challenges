fn main() {
    let age_in_sec = |age: u8| -> u64 { age as u64 * 365 * 24 * 60 * 60 };
    let age = 23;
    println!("{} years --> {} seconds", age, age_in_sec(age));
}
