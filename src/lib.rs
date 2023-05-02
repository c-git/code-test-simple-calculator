//! Assumptions:
//! - Registers cannot have only numeric identifiers
//! - If a register is used on the right side before it is defined then it is created with a 0 value
//! - All operations up to the print should be evaluated when print is called
//! - Registers are never deleted (If that becomes supported then new ID scheme is needed)
//! - Crashing the program is the appropriate response to cycle detection in register evaluation

mod calculator;
mod components;
mod control;

type FnModify = dyn FnOnce(&mut i32, i32);

use calculator::Calculator;
use components::{Operand, Operation, RegID, Register};
pub use control::run;
