fn main() {
    println!("{}", solve(1));
    println!("{}", solve(5));
}

fn solve(program_input: i32) -> i32 {
    let mut ops = get_op_code();

    let mut i = 0;
    let mut program_output = 0;

    loop {
        let mut immediate_mode = (false, false, false);

        let instruction: Opcode = match *ops.get(i).unwrap() {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Save,
            4 => Opcode::Output,
            5 => Opcode::IfTrueJump,
            6 => Opcode::IfFalseJump,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            x => {
                let digits: Vec<u8> = x
                    .to_string()
                    .chars()
                    .map(|n| n.to_digit(10).unwrap() as u8)
                    .rev()
                    .collect::<Vec<u8>>();

                if let Some(mode) = digits.get(2) {
                    immediate_mode.0 = *mode == 1u8;
                }
                if let Some(mode) = digits.get(3) {
                    immediate_mode.1 = *mode == 1u8;
                }
                if let Some(mode) = digits.get(4) {
                    immediate_mode.2 = *mode == 1u8;
                }

                match *digits.get(0).unwrap() {
                    1 => Opcode::Add,
                    2 => Opcode::Multiply,
                    3 => Opcode::Save,
                    4 => Opcode::Output,
                    5 => Opcode::IfTrueJump,
                    6 => Opcode::IfFalseJump,
                    7 => Opcode::LessThan,
                    8 => Opcode::Equals,
                    _ => {
                        panic!("Unknown operation {:?}", x);
                    }
                }
            }
        };
        match instruction {
            Opcode::Add => {
                let (a1, a2, destination) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                    *ops.get(i + 3).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                ops[destination] = num_1 + num_2;
                i += 4;
            }
            Opcode::Multiply => {
                let (a1, a2, destination) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                    *ops.get(i + 3).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                ops[destination] = num_1 * num_2;
                i += 4;
            }
            Opcode::Save => {
                let destination = *ops.get(i + 1).unwrap() as usize;
                ops[destination] = program_input;
                i += 2;
            }
            Opcode::Output => {
                let output = *ops.get(i + 1).unwrap() as usize;
                let value = if immediate_mode.0 {
                    output as i32
                } else {
                    ops[output]
                };

                program_output = value;
                i += 2;
            }
            Opcode::IfTrueJump => {
                let (a1, a2) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                i = if num_1 != 0 { num_2 as usize } else { i + 3 };
            }
            Opcode::IfFalseJump => {
                let (a1, a2) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                i = if num_1 == 0 { num_2 as usize } else { i + 3 };
            }
            Opcode::LessThan => {
                let (a1, a2, destination) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                    *ops.get(i + 3).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                ops[destination] = if num_1 < num_2 { 1 } else { 0 };
                i += 4;
            }
            Opcode::Equals => {
                let (a1, a2, destination) = (
                    *ops.get(i + 1).unwrap() as usize,
                    *ops.get(i + 2).unwrap() as usize,
                    *ops.get(i + 3).unwrap() as usize,
                );

                let num_1 = if immediate_mode.0 { a1 as i32 } else { ops[a1] };
                let num_2 = if immediate_mode.1 { a2 as i32 } else { ops[a2] };

                ops[destination] = if num_1 == num_2 { 1 } else { 0 };
                i += 4;
            }
            Opcode::Halt => break,
        }
    }

    return program_output;
}

fn get_op_code() -> Vec<i32> {
    let input = std::fs::read_to_string("../inputs/5.txt").unwrap();
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
    Save = 3,
    Output = 4,
    IfTrueJump = 5,
    IfFalseJump = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}
