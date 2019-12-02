use std::io::{self, Read};

fn main() {
    let input = get_input();

    // Part 1
    let mut fuel_needed = 0;

    for line in input.lines() {
        let mass: i32 = line.parse().unwrap();
        let fuel = calculate_fuel(mass);
        fuel_needed += fuel;
    }

    println!("[Part One] Fuel needed: {}", fuel_needed);

    // Part 2
    let mut fuel_needed = 0;

    for line in input.lines() {
        let mut mass: i32 = line.parse().unwrap();
        while mass > 0 {
            let fuel = calculate_fuel(mass);
            mass = fuel;
            fuel_needed += fuel;
        }
    }

    println!("[Part Two] Fuel needed: {}", fuel_needed);
}

fn get_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    return input;
}

fn calculate_fuel(mut mass: i32) -> i32 {
    let mut fuel_mass = mass as f32;
    fuel_mass /= 3.0;
    mass = fuel_mass.trunc() as i32;
    mass -= 2;
    return if mass > 0 { mass } else { 0 };
}
