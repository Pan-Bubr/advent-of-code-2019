use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self, Read};

fn main() {
    let (instruction_a, instruction_b) = get_input();

    let mut wire_segments: HashMap<Parameter, StepCounter> = HashMap::new();
    let mut crosses: Vec<(Parameter, StepCounter)> = Vec::new();

    let mut pointer = Parameter { x: 0, y: 0 };
    let mut steps = 0;
    for instruction in instruction_a {
        for _ in 0..instruction.steps {
            steps += 1;
            pointer.step(instruction.direction);

            if let None = wire_segments.get(&pointer) {
                wire_segments.insert(
                    Parameter {
                        x: pointer.x,
                        y: pointer.y,
                    },
                    StepCounter {
                        from_a: steps,
                        from_b: 0,
                    },
                );
            }
        }
    }

    pointer = Parameter { x: 0, y: 0 };
    steps = 0;

    for instruction in instruction_b {
        for _ in 0..instruction.steps {
            steps += 1;
            pointer.step(instruction.direction);

            if let Some(mut counter) = wire_segments.get_mut(&pointer) {
                if counter.from_a != 0 {
                    counter.from_b = if counter.from_b == 0 {
                        steps
                    } else {
                        counter.from_b
                    };
                    crosses.push((
                        Parameter {
                            x: pointer.x,
                            y: pointer.y,
                        },
                        counter.clone(),
                    ));
                }
            } else {
                wire_segments.insert(
                    pointer.clone(),
                    StepCounter {
                        from_a: 0,
                        from_b: steps,
                    },
                );
            }
        }
    }

    let smallest_manhattan = crosses.iter().map(|c| c.0.manhattan()).min().unwrap();
    let smallest_step_count = crosses.iter().map(|c| c.1.step_sum()).min().unwrap();

    println!(
        "[Part One] Min. manhattan distance to crossing is: {}",
        smallest_manhattan
    );
    println!(
        "[Part Two] Min. step count to crossing is: {}",
        smallest_step_count
    );
}

fn get_input() -> (Vec<Instruction>, Vec<Instruction>) {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let instructions: Vec<Vec<Instruction>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|instruction| Instruction {
                    steps: instruction[1..].parse::<i32>().unwrap(),
                    direction: match &instruction[0..1] {
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        _ => panic!("Unknown instruction"),
                    },
                })
                .collect()
        })
        .collect();

    return (instructions[0].clone(), instructions[1].clone());
}

#[derive(Copy, Clone)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Parameter {
    x: i32,
    y: i32,
}

trait Pointer {
    fn manhattan(&self) -> i32;
    fn step(&mut self, _: Direction);
}

impl Pointer for Parameter {
    fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Right => {
                self.x += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        }
    }
}

#[derive(Copy, Clone)]
struct StepCounter {
    from_a: i32,
    from_b: i32,
}

trait Steps {
    fn step_sum(&self) -> i32;
}

impl Steps for StepCounter {
    fn step_sum(&self) -> i32 {
        return self.from_a + self.from_b;
    }
}
