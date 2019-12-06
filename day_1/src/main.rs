use std::io::{self, BufRead};

fn fuel_required(mass: f32) -> i32 {
    return (mass / 3_f32).floor() as i32 - 2;
}

fn main() {
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        let mass = line.unwrap().parse::<f32>().expect("Parsing failed");
        sum += fuel_required(mass);
    }
    println!("{}", sum);
}
