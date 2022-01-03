extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

#[derive(Parser)]
#[grammar = "instructions.pest"]
pub struct InstructionParser;

#[derive(Debug, Clone, Copy)]
struct Register {
    register_w: i32,
    register_x: i32,
    register_y: i32,
    register_z: i32,
}

fn main() {
    let registers_default = Register { register_w: 0, register_x: 0, register_y: 0, register_z: 0 };

    let options = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut input_instruction_steps = vec!();
    
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(instruction_line) = line {
                input_instruction_steps.push(instruction_line);
            }
        }
    }

    let mut inputs = vec![String::from("9"); 14];

    for _i in options {
        let mut registers = registers_default;
        for (offset, input_instruction) in input_instruction_steps.iter().enumerate() {
            // if offset >= 35 {
            //     println!("Instruction: {}, Registers: {:?}", input_instruction, registers);
            //     break;
            // }
            // else {
                println!("{}: {}. {:?}", offset, input_instruction, registers);
                (registers, inputs) = parse_instruction(&registers, input_instruction.to_string(), &inputs);
            // }
        }
    }
}

fn parse_instruction(registers: &Register, instruction_line: String, input: &Vec<String>) -> (Register, Vec<String>) 
{
    let mut output_registers = registers.clone();
    let mut output_inputs = input.clone();

    let parse = InstructionParser::parse(Rule::instruction, &instruction_line).expect("invalid instruction").next().unwrap();
    for instruction_rule in parse.into_inner() {
        // println!("{:?}", instruction_rule);
        match instruction_rule.as_rule() {
            Rule::inp => {
                // println!("{}", instruction_line);
                let input_register = instruction_rule.into_inner().next().unwrap();
                output_registers = input_instruction(&registers, input_register.as_str(), input.iter().next().unwrap());
                output_inputs = output_inputs.drain(1..).collect();
            },
            Rule::add => {
                let mut iterator = instruction_rule.into_inner();
                let register = iterator.next().unwrap();
                let operand = iterator.next().unwrap();
                output_registers = execute_instruction(&registers, register.as_str(), operand.as_str(), "add");
            },
            Rule::mul => {
                let mut iterator = instruction_rule.into_inner();
                let register = iterator.next().unwrap();
                let operand = iterator.next().unwrap();
                output_registers = execute_instruction(&registers, register.as_str(), operand.as_str(), "mul");
            },
            Rule::div => {
                let mut iterator = instruction_rule.into_inner();
                let register = iterator.next().unwrap();
                let operand = iterator.next().unwrap();
                output_registers = execute_instruction(&registers, register.as_str(), operand.as_str(), "div");
            },
            Rule::modulo => {
                let mut iterator = instruction_rule.into_inner();
                let register = iterator.next().unwrap();
                let operand = iterator.next().unwrap();
                output_registers = execute_instruction(&registers, register.as_str(), operand.as_str(), "modulo");
            },
            Rule::eql => {
                let mut iterator = instruction_rule.into_inner();
                let register = iterator.next().unwrap();
                let operand = iterator.next().unwrap();
                output_registers = execute_instruction(&registers, register.as_str(), operand.as_str(), "eql");
            }
            _ => { panic!("Invalid instruction"); }
        }

    }

    (output_registers, output_inputs)
}

fn input_instruction(registers: &Register, register: &str, value_str: &str) -> Register {

    let mut output_registers = registers.clone();

    let value: i32 = value_str.parse().unwrap();
    match register {
        "w" => { output_registers.register_w = value; },
        "x" => { output_registers.register_x = value; },
        "y" => { output_registers.register_y = value; },
        "z" => { output_registers.register_z = value; },
        _ => panic!("Invalid register"),
    }

    output_registers
}

fn execute_instruction(registers: &Register, dest: &str, source: &str, operation: &str) -> Register {
    let mut output_registers = registers.clone();

    let r2: i32 = match source 
    {
        "w" => { registers.register_w },
        "x" => { registers.register_x },
        "y" => { registers.register_y },
        "z" => { registers.register_z },
        _ => source.parse::<i32>().unwrap(),
    };

    let r1  = match dest {
        "w" => { &mut output_registers.register_w },
        "x" => { &mut output_registers.register_x },
        "y" => { &mut output_registers.register_y },
        "z" => { &mut output_registers.register_z},
        _ => { panic!("Invalid r1") }
    };

    match operation {
        "add" => { *r1 += r2 },
        "mul" => { *r1 *= r2 },
        "modulo" => { *r1 %= r2},
        "div" => { *r1 /= r2 },
        "eql" => { if *r1 == r2 { *r1 = 1 } else { *r1 = 0 } },
        _ => { panic!("Invalid operation") },
    };

    output_registers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test_instruction() {
    let mut registers = Register { register_w: 2, register_x: 3, register_y: 2, register_z: 0 };

    registers = execute_instruction(&registers, "w", "x", "add");
    registers = execute_instruction(&registers, "y", "-12", "mul");

    assert_eq!(registers.register_w, 5);
    assert_eq!(registers.register_y, -24);
}

#[test]
fn test_input_instruction() {
    let mut registers = Register { register_w: 2, register_x: 3, register_y: 2, register_z: 0 };

    registers = input_instruction(&registers, "w", "12");
    registers = input_instruction(&registers, "y", "-12");

    assert_eq!(registers.register_w, 12);
    assert_eq!(registers.register_y, -12);
}

#[test]
fn test_sample_instructions() {
    let input_instrutions = vec!["inp w",
                                        "add z w", 
                                        "mod z 2", 
                                        "div w 2", 
                                        "add y w", 
                                        "mod y 2", 
                                        "div w 2", 
                                        "add x w", 
                                        "mod x 2", 
                                        "div w 2", 
                                        "mod w 2"];

    let mut registers = Register{ register_w: 0, register_x: 0, register_y: 0, register_z: 0 };
    let mut inputs = vec![String::from("15"), String::from("2")];

    for i in input_instrutions.iter() {
        (registers, inputs) = parse_instruction(&registers, i.to_string(), &inputs);
        println!("Instruction: {}, Registers: {:?}", i, registers);
    }

    assert_eq!(registers.register_w, 1);
    assert_eq!(registers.register_x, 1);
    assert_eq!(registers.register_y, 1);
    assert_eq!(registers.register_z, 1);

    assert_eq!(inputs, vec!(String::from("2")));

    registers = Register{ register_w: 0, register_x: 0, register_y: 0, register_z: 0 };
    inputs = vec![String::from("10")];
    
    for i in input_instrutions.iter() {
        (registers, inputs) = parse_instruction(&registers, i.to_string(), &inputs);
        println!("Instruction: {}, Registers: {:?}", i, registers);
    }

    assert_eq!(registers.register_w, 1);
    assert_eq!(registers.register_x, 0);
    assert_eq!(registers.register_y, 1);
    assert_eq!(registers.register_z, 0);

}