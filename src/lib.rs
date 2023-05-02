//! Assumptions:
//! - Registers cannot have only numeric identifiers
//! - If a register is used on the right side before it is defined then it is created with a 0 value
//! - All operations up to the print should be evaluated when print is called
//! - Registers are never deleted (If that becomes supported then new ID scheme is needed)

use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
    rc::Rc,
};

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
                    match calculator.get_register_value(parts[1]) {
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
enum Operand {
    Register(RegID),
    Number(i32),
}
impl Operand {
    fn new(value: &str) -> Self {
        if let Ok(number) = value.parse::<i32>() {
            Self::Number(number)
        } else {
            Self::Register(
                RegID::new(value.to_owned())
                    .expect("Internal Error: This should have been a valid Register Name"),
            )
        }
    }
}

enum Operation {
    Add(RegID, Operand),
    Subtract(RegID, Operand),
    Multiply(RegID, Operand),
}

impl Operation {
    fn new(operand1: &str, op: &str, operand2: &str) -> Result<Self, String> {
        match op {
            "add" => {
                let reg = RegID::new(operand1.to_owned())?;
                let operand2 = Operand::new(operand2);
                Ok(Operation::Add(reg, operand2))
            }
            "subtract" => {
                let reg = RegID::new(operand1.to_owned())?;
                let operand2 = Operand::new(operand2);
                Ok(Operation::Subtract(reg, operand2))
            }
            "multiply" => {
                let reg = RegID::new(operand1.to_owned())?;
                let operand2 = Operand::new(operand2);
                Ok(Operation::Multiply(reg, operand2))
            }
            _ => Err(format!("Unexpected operation found {op}")),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct RegID(Rc<String>);

impl RegID {
    fn new(name: String) -> Result<Self, String> {
        // Ensure name is not only numeric
        if name.parse::<i32>().is_ok() {
            return Err(format!(
                "Register names are not allowed to be only numeric but got: {name}"
            ));
        }

        Ok(Self(Rc::new(name)))
    }
}

struct Calculator {
    registers: HashMap<RegID, i32>, // Name to value
    pending_operations: VecDeque<Operation>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            registers: Default::default(),
            pending_operations: Default::default(),
        }
    }

    fn get_register_value(&mut self, register_name: &str) -> Result<i32, String> {
        self.perform_operations();
        debug_assert!(self.pending_operations.is_empty());
        let id = RegID::new(register_name.to_owned())?;
        Ok(*self.registers.entry(id).or_default())
    }

    fn operation(&mut self, operand1: &str, op: &str, operand2: &str) -> Result<(), String> {
        let new_op = Operation::new(operand1, op, operand2)?;
        self.pending_operations.push_back(new_op);
        Ok(())
    }

    fn perform_operations(&mut self) {
        while !self.pending_operations.is_empty() {
            let op = self
                .pending_operations
                .pop_front()
                .expect("Only enters loop if has a value");
            self.execute(op);
        }
    }

    fn execute(&mut self, op: Operation) {
        type FnModify = dyn FnOnce(&mut i32, i32);
        let mut binary_op = |reg_id: RegID, operand2: Operand, f: Box<FnModify>| {
            let value2 = match operand2 {
                Operand::Register(reg_id) => *self.registers.entry(reg_id).or_default(),
                Operand::Number(val) => val,
            };
            self.registers.entry(reg_id).and_modify(|x| f(x, value2));
        };

        match op {
            Operation::Add(reg_id, operand2) => {
                binary_op(reg_id, operand2, Box::new(|reg, val| *reg += val));
            }
            Operation::Subtract(reg_id, operand2) => {
                binary_op(reg_id, operand2, Box::new(|reg, val| *reg -= val));
            }
            Operation::Multiply(reg_id, operand2) => {
                binary_op(reg_id, operand2, Box::new(|reg, val| *reg *= val));
            }
        }
    }
}
