use std::time::SystemTime;

fn calc(mut index: u64) -> u64 {
    let mut step = 0;
    while index > 1 {
        if index % 2 == 0 {
            index /= 2;
            step += 1;
        } else {
            index = (3 * index + 1) / 2;
            step += 2;
        }
    }
    step
}

fn main() {
    let sys_time = SystemTime::now();
    let mut result = 0;
    for i in 1..1_000_001 {
        result = result.max(calc(i));
    }
    println!("{:?}", result);
    let elapsed = sys_time.elapsed().unwrap();
    
    println!("elapsed time: {:?}", elapsed);
}