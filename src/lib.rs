use std::io::{self, BufRead};

pub fn run(input: &mut dyn BufRead) -> io::Result<()> {
    let mut registers = Registers::new();

    for line in input.lines() {
        let line = line?.trim().to_lowercase();
        if line == "quit" {
            break;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        match parts.len() {
            2 => {
                if parts[0] != "print" {
                    eprintln!("Got to parts but first is not print. '{line}'");
                } else {
                    match registers.get_register_value(parts[1]) {
                        Ok(value) => println!("{value}"),
                        Err(e) => eprintln!("Evaluation of operations resulted in an error: {e}"),
                    }
                }
            }
            3 => {
                let (operand1, op, operand2) = (parts[0], parts[1], parts[2]);
                if let Err(e) = registers.operation(operand1, op, operand2) {
                    eprintln!("Error on line '{line}' Error: {e}");
                }
            }
            _ => eprintln!("Got unexpected input: '{line}'"),
        }
    }
    Ok(())
}

struct Registers {}

impl Registers {
    fn new() -> Self {
        Self {}
    }

    fn get_register_value(&mut self, register_name: &str) -> Result<i32, String> {
        todo!()
    }

    fn operation(&mut self, operand1: &str, op: &str, operand2: &str) -> Result<(), String> {
        todo!()
    }
}
