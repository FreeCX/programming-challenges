fn cesar_encrypt(text: &str, shift: u8) -> String {
    let n = 'z' as u8 - 'a' as u8 + 1;
    let mut buffer = String::with_capacity(n as usize);
    for character in text.to_lowercase().chars() {
        buffer.push(match character {
            'a'...'z' => (((character as u8 - 'a' as u8 + shift) % n) + 'a' as u8) as char,
            _ => character
        });
    }
    buffer
}

fn cesar_decrypt(text: &str, shift: u8) -> String {
    let shift = 'z' as u8 - 'a' as u8 + 1 - shift;
    cesar_encrypt(text, shift)
}

#[test]
fn cesar_test() {
    let text = "this is simple text for cesar encryption/decryption algorithm";
    let shift = 3;
    let encrypt = cesar_encrypt(text, shift);
    let decrypt = cesar_decrypt(&encrypt, shift);
    assert_eq!(text, decrypt)
}

fn main() {
    let text = "test string z";
    let shift = 1;
    let encrypt = cesar_encrypt(text, shift);
    let decrypt = cesar_decrypt(&encrypt, shift);
    println!("`{}` --> `{}` --> `{}`", text, encrypt, decrypt);
}