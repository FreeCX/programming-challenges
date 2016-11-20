fn main() {
    let p = 34 as char;
    let s = "fn main() {
    let p = 34 as char;
    let s = {};
    print!({}{}{}{}{}{}{}{}{}, &s[..48], p, s, p, &s[50..63], p, &s[63..81], p, &s[81..]);
}";
    print!("{}{}{}{}{}{}{}{}{}", &s[..48], p, s, p, &s[50..63], p, &s[63..81], p, &s[81..]);
}