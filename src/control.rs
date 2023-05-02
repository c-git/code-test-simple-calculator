use std::io::{self, BufRead};

use crate::{components::RegID, Calculator};

pub fn run(input: &mut dyn BufRead) -> io::Result<()> {
    let mut calculator = Calculator::new();

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
                    let reg_id = match RegID::new(parts[1].to_string()) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    };
                    match calculator.get_register_value(reg_id) {
                        Ok(value) => println!("{value}"),
                        Err(e) => eprintln!("Evaluation of operations resulted in an error: {e}"),
                    }
                }
            }
            3 => {
                let (operand1, op, operand2) = (parts[0], parts[1], parts[2]);
                if let Err(e) = calculator.operation(operand1, op, operand2) {
                    eprintln!("Error on line '{line}' Error: {e}");
                }
            }
            _ => eprintln!("Got unexpected input: '{line}'"),
        }
    }
    Ok(())
}
