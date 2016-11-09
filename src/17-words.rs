fn main() {
    let string: &'static str = "The quick brown fox jumps over the lazy dog";
    let word_count = string.split(' ').collect::<Vec<_>>().len();
    println!("string = `{}`", string);
    println!(" count = {}", word_count);
}