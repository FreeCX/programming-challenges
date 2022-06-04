#![allow(dead_code)]
pub mod pseudo;

use pseudo::Rng;
use std::cmp::Ordering;
use std::io;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Eq, Copy, Clone)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Figure {
    fn cmp(&self, other: &Figure) -> Ordering {
        use Figure::*;
        match (self, other) {
            (&Rock, &Rock) => Ordering::Equal,
            (&Rock, &Paper) => Ordering::Less,
            (&Rock, &Scissors) => Ordering::Greater,
            (&Paper, &Rock) => Ordering::Greater,
            (&Paper, &Paper) => Ordering::Equal,
            (&Paper, &Scissors) => Ordering::Less,
            (&Scissors, &Rock) => Ordering::Less,
            (&Scissors, &Paper) => Ordering::Greater,
            (&Scissors, &Scissors) => Ordering::Equal,
        }
    }
}

impl PartialEq for Figure {
    fn eq(&self, other: &Figure) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Figure {
    fn partial_cmp(&self, other: &Figure) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Figure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Figure::*;
        match s.trim() {
            "r" | "rock" => Ok(Rock),
            "p" | "paper" => Ok(Paper),
            "s" | "scissors" => Ok(Scissors),
            figure => Err(format!("unknown `{}` figure", figure)),
        }
    }
}

impl Figure {
    fn select(n: u8) -> Figure {
        match n {
            0 => Figure::Rock,
            1 => Figure::Paper,
            2 => Figure::Scissors,
            index => panic!("something went wrong: unknown figure index ({})!", index),
        }
    }
}

fn main() {
    let time = SystemTime::now().elapsed().unwrap();
    let mut rng = Rng::new(time.subsec_nanos());
    let mut buffer = String::new();
    loop {
        let computer_figure = Figure::select((rng.rand() % 3) as u8);
        let mut user_figure = Figure::Rock;
        'select_loop: loop {
            println!("[?] Rock, paper or scissors ?");
            io::stdin().read_line(&mut buffer).unwrap();
            match Figure::from_str(&buffer.to_lowercase()) {
                Ok(figure) => {
                    user_figure = figure;
                    break 'select_loop;
                }
                Err(err) => println!("[error]: {}", err),
            }
            buffer.clear();
        }
        let result = if computer_figure > user_figure {
            "computer wins!"
        } else if computer_figure == user_figure {
            "draw"
        } else {
            "user wins!"
        };
        println!(">>> {:?} vs {:?}: {}", computer_figure, user_figure, result);
        buffer.clear();
        println!("[?] Again (y|yes|n|no)?");
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.to_lowercase().trim() {
            "n" | "no" => break,
            _ => {}
        };
        buffer.clear();
    }
}
