use std::{collections::VecDeque, rc::Rc};

use crate::FnModify;

#[derive(Debug)]
pub(crate) enum Operation {
    Add(RegID, Operand),
    Subtract(RegID, Operand),
    Multiply(RegID, Operand),
}

impl Operation {
    pub fn new(operand1: &str, op: &str, operand2: &str) -> Result<Self, String> {
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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub(crate) struct RegID(Rc<String>);

impl RegID {
    pub fn new(name: String) -> Result<Self, String> {
        // Ensure name is not only numeric
        if name.parse::<i32>().is_ok() {
            return Err(format!(
                "Register names are not allowed to be only numeric but got: {name}"
            ));
        }

        Ok(Self(Rc::new(name)))
    }
}

#[derive(Debug)]
pub(crate) enum Operand {
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

#[derive(Debug)]
pub(crate) struct Register {
    id: RegID,
    value: i32,
    pending_transactions: VecDeque<Operation>,
}

impl Register {
    pub(crate) fn new(id: RegID) -> Self {
        Self {
            id,
            value: Default::default(),
            pending_transactions: Default::default(),
        }
    }

    pub fn add_pending_op(&mut self, operation: Operation) {
        debug_assert_eq!(
            match &operation {
                Operation::Add(reg_id, _)
                | Operation::Subtract(reg_id, _)
                | Operation::Multiply(reg_id, _) => reg_id,
            },
            &self.id,
            "Can only add operations that change this register"
        );
        self.pending_transactions.push_back(operation);
    }

    pub fn get_pending_op(&mut self) -> Option<Operation> {
        self.pending_transactions.pop_front()
    }

    pub fn get_value(&mut self) -> i32 {
        debug_assert!(
            self.pending_transactions.is_empty(),
            "Only able to access value when all pending transactions have been processed"
        );
        self.value
    }

    pub(crate) fn perform(&mut self, f: Box<FnModify>, value2: i32) {
        f(&mut self.value, value2);
    }
}
