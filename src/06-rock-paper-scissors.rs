#![allow(dead_code)]
pub mod pseudo;

use std::time::SystemTime;
use std::cmp::Ordering;
use std::str::FromStr;
use pseudo::Rng;
use std::io;

#[derive(Debug, Eq, Copy, Clone)]
enum Figure {
    Rock,
    Paper,
    Scissors
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
            (&Scissors, &Scissors) => Ordering::Equal
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Figure::*;
        match s.trim() {
            "r" | "rock" => Ok(Rock),
            "p" | "paper" => Ok(Paper),
            "s" | "scissors" => Ok(Scissors),
            _ => Err(())
        }
    }
}

impl Figure {
    fn select(n: u8) -> Figure {
        match n {
            0 => Figure::Rock,
            1 => Figure::Paper,
            2 => Figure::Scissors,
            _ => panic!("unknown figure index")
        }
    }
}

fn main() {
    let time = SystemTime::now().elapsed().unwrap();
    let mut rng = Rng::new(time.subsec_nanos());
    let mut buffer = String::new();
    loop {
        let computer_figure = Figure::select((rng.rand() % 3) as u8);
        println!("[?] Rock, paper or scissors ?");
        io::stdin().read_line(&mut buffer)
                   .unwrap();
        let user_figure = Figure::from_str(&buffer.to_lowercase()).unwrap();
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
        io::stdin().read_line(&mut buffer)
                   .unwrap();
        match buffer.to_lowercase().trim() {
            "n" | "no" => break,
            _ => {}
        };
        buffer.clear();
    }
}
