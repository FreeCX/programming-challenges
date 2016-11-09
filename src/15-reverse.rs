fn main() {
    let string: &'static str = "The quick brown fox jumps over the lazy dog";
    let reverse: String = string.chars().rev().collect();
    println!("  normal = `{}`", string);
    println!("reversed = `{}`", reverse);
}