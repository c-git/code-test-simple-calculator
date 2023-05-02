use std::collections::{HashMap, HashSet};

use crate::{FnModify, Operand, Operation, RegID, Register};

pub(crate) struct Calculator {
    registers: HashMap<RegID, Register>,
    eval_in_progress: HashSet<RegID>, // Used for cycle detection when resolving registers
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            registers: Default::default(),
            eval_in_progress: Default::default(),
        }
    }

    pub fn get_register_value(&mut self, reg_id: RegID) -> Result<i32, String> {
        assert!(
            !self.eval_in_progress.contains(&reg_id),
            "Cycle detected with the following registers: {:?}",
            self.eval_in_progress
        );

        self.eval_in_progress.insert(reg_id.clone());

        // Ensure register is created so that this borrow can be released
        let register = self
            .registers
            .entry(reg_id.clone())
            .or_insert_with_key(|id| Register::new(id.clone()));

        let mut next_op = register.get_pending_op();

        while let Some(op) = next_op {
            // Execute operation
            let mut binary_op =
                |reg_id: RegID, operand2: Operand, f: Box<FnModify>| -> Result<(), String> {
                    let value2 = match operand2 {
                        Operand::Register(other_id) => self.get_register_value(other_id)?,
                        Operand::Number(val) => val,
                    };

                    self.registers
                        .get_mut(&reg_id)
                        .expect("Inserted at top of function")
                        .perform(f, value2);
                    Ok(())
                };

            match op {
                Operation::Add(reg_id, operand2) => {
                    binary_op(reg_id, operand2, Box::new(|reg, val| *reg += val))?
                }
                Operation::Subtract(reg_id, operand2) => {
                    binary_op(reg_id, operand2, Box::new(|reg, val| *reg -= val))?
                }
                Operation::Multiply(reg_id, operand2) => {
                    binary_op(reg_id, operand2, Box::new(|reg, val| *reg *= val))?
                }
            }

            next_op = self
                .registers
                .get_mut(&reg_id)
                .expect("Inserted at top of function")
                .get_pending_op();
        }

        self.eval_in_progress.remove(&reg_id);

        Ok(self
            .registers
            .get_mut(&reg_id)
            .expect("Inserted at top of function")
            .get_value())
    }

    pub fn operation(&mut self, operand1: &str, op: &str, operand2: &str) -> Result<(), String> {
        let new_op = Operation::new(operand1, op, operand2)?;

        // Add to the correct pending list (right now all get added to operand1)
        match new_op {
            Operation::Add(ref reg_id, _)
            | Operation::Subtract(ref reg_id, _)
            | Operation::Multiply(ref reg_id, _) => self
                .registers
                .entry(reg_id.clone())
                .or_insert_with_key(|id| Register::new(id.clone()))
                .add_pending_op(new_op),
        }
        Ok(())
    }
}
