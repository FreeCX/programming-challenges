pub mod pseudo;

use pseudo::Rng;
use std::io;
use std::time::SystemTime;

fn main() {
    let time = SystemTime::now().elapsed().unwrap();
    let mut rng = Rng::new(time.subsec_nanos());
    let mut buffer = String::new();
    loop {
        let coin = rng.rand() % 2;
        println!("[?] Head (h|head) or Tail (t|tail) ?");
        io::stdin().read_line(&mut buffer).unwrap();
        let select = match buffer.to_lowercase().trim() {
            "h" | "head" => Some(0),
            "t" | "tail" => Some(1),
            _ => None,
        };
        match select {
            Some(v) => println!("[!] You {}!", if v == coin { "win" } else { "lose" }),
            None => println!("[!] Unknown command `{}`! Please use h|head|t|tail!", buffer.trim()),
        };
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
