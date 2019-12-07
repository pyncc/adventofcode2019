use std::io::{self, BufRead};
use std::cmp::Ordering;


fn fuel_required_fuel(fuel_mass: i32) -> i32 {
    let fuel_fuel_mass: i32 = (fuel_mass as f32 / 3_f32).floor() as i32 - 2;
    // println!("fuel {}, extra {}", fuel_mass, fuel_fuel_mass);
    return fuel_fuel_mass + match fuel_fuel_mass.cmp(&0) {
        Ordering::Less => -1 * fuel_fuel_mass,
        Ordering::Equal => 0i32,
        Ordering::Greater => fuel_required_fuel(fuel_fuel_mass)
    };
}

fn fuel_required_module(mass: f32) -> i32 {
    return (mass / 3_f32).floor() as i32 - 2;
}

fn main() {
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        let mass = line.unwrap().parse::<f32>().expect("Parsing failed");
        let module_fuel_mass = fuel_required_module(mass);
        let fuel_fuel_mass = fuel_required_fuel(module_fuel_mass);
        //println!("module {}, fuel {}", module_fuel_mass, fuel_fuel_mass);
        sum += module_fuel_mass + fuel_fuel_mass;
    }
    println!("{}", sum);
}
