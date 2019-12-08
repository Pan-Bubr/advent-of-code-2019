use std::io::{self, Read};

fn main() {
    let mut ops = get_input();

    ops[1] = 1;
    ops[2] = 12;

    for i in (0..).step_by(4) {
        let (a1, a2, destination) = (
            *ops.get(i + 1).unwrap() as usize,
            *ops.get(i + 2).unwrap() as usize,
            *ops.get(i + 3).unwrap() as usize,
        );

        let instruction = match *ops.get(i).unwrap() {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            x => panic!("Unknown operation {}", x),
        };

        match instruction {
            Opcode::Add => ops[destination] = ops[a1] + ops[a2],
            Opcode::Multiply => ops[destination] = ops[a1] * ops[a2],
            Opcode::Halt => break,
        }
    }

    println!("[Part One] State of the computer: {}", ops.get(0).unwrap());
}

fn get_input() -> Vec<i32> {
    let input = std::fs::read_to_string("../inputs/4.txt").unwrap();
    let numbers: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    return numbers;
}

#[derive(Copy, Clone)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}
