use std::io::{self, Read};

fn main() {
    let mut ops = get_input();

    if let Some([el1, el2]) = ops.get_mut(1..3) {
        *el1 = 1; 
        *el2 = 2;
    }

    let mut pointer = 0;
    loop {
        match ops.get(pointer) {
            Some(1) => {
                add(&mut ops, pointer);
                pointer += 4;
            },
            Some(2) => {
                multiply(&mut ops, pointer);
                pointer += 4;
            },
            _ => {
                println!("[Part One] State of the computer: {}", ops.get(0).unwrap());
                break;
            }
        }
    }
}

fn add(ops: &mut Vec<i32>, pointer: usize) {
    let (a1, a2, destination) = match ops.get(pointer+1..pointer+4) {
        Some([a1, a2, destination]) => (*a1 as usize, *a2 as usize, *destination as usize),
        _ => { panic!("Address invalid") }
    };

    let num1: i32 = match ops.get(a1) {
        Some(n) => *n,
        _ => { panic!("Address invalid") }
    };

    let num2: i32 = match ops.get(a2) {
        Some(n) => *n,
        _ => { panic!("Address invalid") }
    };

    if let Some(n) = ops.get_mut(destination) {
        *n = num1 + num2;
    }
}

fn multiply(ops: &mut Vec<i32>, pointer: usize) {
    let (a1, a2, destination) = match ops.get(pointer+1..pointer+4) {
        Some([a1, a2, destination]) => (*a1 as usize, *a2 as usize, *destination as usize),
        _ => { panic!() }
    };

    let num1: i32 = match ops.get(a1) {
        Some(n) => *n,
        _ => { panic!("Address invalid") }
    };
    
    let num2: i32 = match ops.get(a2) {
        Some(n) => *n,
        _ => { panic!("Address invalid") }
    };

    if let Some(n) = ops.get_mut(destination) {
        *n = num1 * num2;
    }
}

fn get_input() -> Vec<i32> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    return numbers;
}
