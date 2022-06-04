use std::io::{self, Read, Write};

#[derive(Clone, Copy)]
pub enum Command {
    Next,
    Previous,
    Increment,
    Decrement,
    Put,
    Read,
    LoopBegin(usize),
    LoopEnd(usize),
}

pub struct Emulator {
    stack: Vec<usize>,
    app: Vec<Command>,
    mem: Vec<u8>,
    ip: usize,
    mp: usize,
}

impl Emulator {
    pub fn new(mem_size: usize) -> Emulator {
        Emulator { stack: Vec::new(), mem: vec![0; mem_size], app: Vec::new(), ip: 0, mp: 0 }
    }

    fn parse_token(&mut self, token: char) -> Result<(), String> {
        if Command::is_valid_token(token) {
            let index = self.app.len();
            let cmd = match token.into() {
                command @ Command::LoopBegin(_) => {
                    self.stack.push(index);
                    command
                }
                Command::LoopEnd(_) => {
                    let start = match self.stack.pop() {
                        Some(value) => value,
                        None => return Err("loop start not found".to_string()),
                    };
                    self.app[start] = Command::LoopBegin(index);
                    Command::LoopEnd(start)
                }
                cmd => cmd,
            };
            self.app.push(cmd);
        }

        Ok(())
    }

    pub fn from_buffer(&mut self, buffer: &str) -> Result<(), String> {
        for token in buffer.chars() {
            self.parse_token(token)?;
        }

        match !self.stack.is_empty() {
            true => Err("number of brackets does not match".to_string()),
            false => Ok(()),
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        match self.app[self.ip] {
            Command::Next => {
                if self.mp + 1 == self.mem.len() {
                    return Err("out of memory".to_string());
                }
                self.mp += 1;
            }
            Command::Previous => {
                if self.mp == 0 {
                    return Err("cannot access negative memory index".to_string());
                }
                self.mp -= 1;
            }
            Command::Increment => self.mem[self.mp] = self.mem[self.mp].overflowing_add(1).0,
            Command::Decrement => self.mem[self.mp] = self.mem[self.mp].overflowing_sub(1).0,
            Command::Put => {
                print!("{}", self.mem[self.mp] as char);
                let _ = io::stdout().flush();
            }
            Command::Read => self.mem[self.mp] = io::stdin().bytes().next().unwrap().unwrap(),
            Command::LoopBegin(index) => {
                if self.mem[self.mp] == 0 {
                    self.ip = index;
                }
            }
            Command::LoopEnd(index) => {
                if self.mem[self.mp] != 0 {
                    self.ip = index;
                }
            }
        }
        self.ip += 1;

        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while self.ip < self.app.len() {
            self.step()?;
        }

        Ok(())
    }
}

impl From<char> for Command {
    fn from(token: char) -> Self {
        match token {
            '>' => Command::Next,
            '<' => Command::Previous,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            '.' => Command::Put,
            ',' => Command::Read,
            '[' => Command::LoopBegin(0),
            ']' => Command::LoopEnd(0),
            token => panic!("unknown token `{}`", token),
        }
    }
}

impl Command {
    pub fn is_valid_token(t: char) -> bool {
        t == '>' || t == '<' || t == '+' || t == '-' || t == '.' || t == ',' || t == '[' || t == ']'
    }
}

fn main() {
    let hello_world = r#"
        ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++
        ..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;
    let mut machine = Emulator::new(30_000);
    let _ = machine.from_buffer(hello_world);
    let _ = machine.execute();
}
