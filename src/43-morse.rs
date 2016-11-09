// sound generation
// - https://freecx.github.io/blog/2016/09/07/sound-generator-for-morse (RUS)
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref ERROR_CODE: &'static str = "········";
    static ref END_TRANSMISSION: &'static str = "··-·-";
    static ref MORSE_TABLE: Vec<(&'static str, &'static str)> = vec![
        ("А", "·-"), ("Б", "-···"), ("В", "·--"), ("Г", "--·"), ("Д", "-··"), ("Ё", "·"), ("Е", "·"),
        ("Ж", "···-"), ("З", "--··"), ("И", "··"), ("Й", "·---"), ("К", "-·-"), ("Л", "·-··"), ("М", "--"), 
        ("Н", "-·"), ("О", "---"), ("П", "·--·"), ("Р", "·-·"), ("С", "···"), ("Т", "-"), ("У", "··-"), 
        ("Ф", "··-·"), ("Х", "····"), ("Ц", "-·-·"), ("Ч", "---·"), ("Ш", "----"), ("Щ", "--·-"), 
        ("Ъ", "--·--"), ("Ы", "-·--"), ("Ь", "-··-"), ("Э", "··-··"), ("Ю", "··--"), ("Я", "·-·-"), 
        ("1", "·----"), ("2", "··---"), ("3", "···--"), ("4", "····-"), ("5", "·····"), ("6", "-····"), 
        ("7", "--···"), ("8", "---··"), ("9", "----·"), ("0", "-----"), (".", "······"), (",", "·-·-·-"), 
        (":", "---···"), (";", "-·-·-·"), (")", "-·--·-"), ("(", "-·--·-"), ("'", "·----·"), 
        ("\"", "·-··-·"), ("-", "-····-"), ("\\", "-··-·"), ("/", "-··-·"), ("!", "··--··"), 
        ("?", "--··--"), (" ", "-···-"), ("@", "·--·-·")
        // ("A", "·-"), ("B", "-···"), ("C", "-·-·"), ("D", "-··"), ("E", "."), ("F", "··-·"), ("G", "--·"), 
        // ("H", "····"), ("I", "··"), ("J", "·---"), ("K", "-·-"), ("L", "·-··"), ("M", "--"), ("N", "-·"), 
        // ("P", "---"), ("P", "·--·"), ("Q", "--·-"), ("R", "·-·"), ("S", "···"), ("T", "-"), ("U", "··-"), 
        // ("V", "···-"), ("W", "·--"), ("X", "-··-"), ("Y", "-·--"), ("Z", "--··")
    ];
    static ref CODER: HashMap<String, String> = {
        let mut hashmap = HashMap::new();
        for &(symbol, morse) in MORSE_TABLE.iter() {
            hashmap.insert(symbol.to_string(), morse.to_string());
        }
        hashmap
    };
    static ref DECODER: HashMap<String, String> = {
        let mut hashmap = HashMap::new();
        for &(symbol, morse) in MORSE_TABLE.iter() {
            hashmap.insert(morse.to_string(), symbol.to_string());
        }
        hashmap.insert(END_TRANSMISSION.to_string(), "<КОНЕЦ СВЯЗИ>".to_string());
        hashmap.insert(ERROR_CODE.to_string(), "<ОШИБКА/ПЕРЕБОЙ>".to_string());
        hashmap
    };
}

struct MorseConverter {
    text: String
}

impl MorseConverter {
    fn new() -> MorseConverter {
        MorseConverter { text: String::new() }
    }
    fn decode(input: &str) -> String {
        let mut result = String::new();
        for code in input.split(' ') {
            match DECODER.get(code) {
                Some(symb) => result.push_str(&symb),
                None => ()
            };
        }
        result
    }
    fn push_str(&mut self, input: &str) {
        let input_str = input.to_uppercase();
        for symbol in input_str.chars() {
            match CODER.get(&format!("{}", symbol)) {
                Some(code) => self.text.push_str(&format!("{} ", code)),
                None => ()
            };
        }
    }
    fn return_error_code() -> String {
        format!("{}", ERROR_CODE.to_string())
    }
    fn end_transmission(&self) -> String {
        format!("{}{}", self.text, END_TRANSMISSION.to_string())
    }
}

fn main() {
    let mut coder = MorseConverter::new();
    coder.push_str("привет медведь");
    println!("{}", MorseConverter::decode(&coder.end_transmission()));
}