extern crate time;

use std::io::{Read, Result};
use std::fs::File;

struct Digits {
    size: (usize, usize),
    alphabet: Vec<char>,
    digits: Vec<String>
}

impl Digits {
    fn new(filename: &str) -> Result<Digits> {
        let mut file = File::open(filename).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let digit_size: Vec<usize> = buffer.lines()
                                        .nth(0).unwrap().split('x')
                                        .map(|x| x.parse().unwrap()).collect();
        let alphabet: Vec<char> = buffer.lines()
                                        .nth(1).unwrap()
                                        .chars().collect();
        let mut digits: Vec<String> = Vec::new();
        for line in buffer.lines().skip(2) {
            digits.push(line.to_owned());
        }
        Ok(Digits {
            size: (digit_size[0], digit_size[1]),
            alphabet, digits
        })
    }

    fn print_text(&self, text: &str) {
        let mut buffer = String::new();
        let indexes: Vec<usize> = text.chars()
                                      .map(|x| self.alphabet.binary_search(&x).unwrap())
                                      .collect();
        for line in &self.digits {
            for index in &indexes {
                let start_x = (self.size.0 + 1) * index;
                let stop_x = (self.size.0 + 1) * (index + 1) - 1;
                buffer.push_str(&line[start_x..stop_x]);
                buffer.push(' ');
            }
            buffer.push('\n');
        }
        buffer.pop().unwrap();
        println!("{}", buffer);
    }
}

fn main() {
    let d = Digits::new("./assets/digits.txt").unwrap();
    let time_now = time::now();
    let time_str = time::strftime("%H:%M:%S", &time_now).unwrap();
    d.print_text(&time_str);
}