use std::fs;

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<u8>,
    a: u64,
    b: u64,
    c: u64,
    pointer: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("works");
    let program = extract_program(&input);
    let result_part1 = run_program(program.clone());
    let result_part2 = find_copy(program.clone(), program.instructions.len() - 1, 0);
    println!(
        "part1: {}",
        result_part1
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    println!("part2: {}", result_part2);
}

fn find_copy(mut program: Program, cursor: usize, sofar: u64) -> u64 {
    for i in 0..8 {
        program.a = sofar * 8 + i;
        if run_program(program.clone()) == program.instructions[cursor..] {
            if cursor == 0 {
                return program.a;
            }
            let result = find_copy(program.clone(), cursor - 1, program.a);
            if result != 0 {
                return result;
            }
        }
    }
    0
}

fn run_program(mut program: Program) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    while program.pointer < program.instructions.len() {
        let instruction = program.instructions[program.pointer];
        let operand = program.instructions[program.pointer + 1];
        program.pointer += 2;
        if instruction == 0 {
            program.a = program.a / 2u64.pow(combo_for(operand, &program) as u32);
        } else if instruction == 1 {
            program.b = program.b ^ operand as u64;
        } else if instruction == 2 {
            program.b = combo_for(operand, &program) % 8;
        } else if instruction == 3 && program.a != 0 {
            program.pointer = operand as usize;
        } else if instruction == 4 {
            program.b = program.b ^ program.c;
        } else if instruction == 5 {
            output.push((combo_for(operand, &program) % 8) as u8);
        } else if instruction == 6 {
            program.b = program.a / 2u64.pow(combo_for(operand, &program) as u32);
        } else if instruction == 7 {
            program.c = program.a / 2u64.pow(combo_for(operand, &program) as u32);
        }
    }
    output
}

fn combo_for(operand: u8, program: &Program) -> u64 {
    if operand < 4 {
        operand.into()
    } else if operand == 4 {
        program.a
    } else if operand == 5 {
        program.b
    } else {
        program.c
    }
}

fn extract_program(input: &str) -> Program {
    let pointer: usize = 0;
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut c: u64 = 0;
    let mut instructions: Vec<u8> = vec![];
    for line in input.lines() {
        if let Some((_, value)) = line.split_once(":") {
            println!("{}", value);
            if line.contains("A") {
                a = value.trim().parse::<u64>().unwrap();
            } else if line.contains("B") {
                b = value.trim().parse::<u64>().unwrap();
            } else if line.contains("C") {
                c = value.trim().parse::<u64>().unwrap();
            } else {
                instructions = value
                    .trim()
                    .split(',')
                    .map(|value| value.parse::<u8>().unwrap())
                    .collect();
            }
        }
    }
    Program {
        instructions,
        a,
        b,
        c,
        pointer,
    }
}
