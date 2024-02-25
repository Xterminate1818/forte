use std::fmt::{Debug, Display};

use crate::{op::Op, Word};

pub fn abs_sub(a: Word, b: Word) -> Word {
  let max = u128::max(a, b);
  let min = u128::min(a, b);
  max - min
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
  StackOverflow,
  StackUnderflow,
  StackUnbalanced,
  WriteOob,
  ReadOob,
  BadCall,
  NoExec,
  IllegalOp,
  NoFunc,
  ExecLimit,
  EndReached,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::StackOverflow => write!(f, "Stack Overflow"),
      Self::StackUnderflow => write!(f, "Stack Underflow"),
      Self::StackUnbalanced => write!(f, "Stack Unbalanced"),
      Self::WriteOob => write!(f, "Memory write out of bounds"),
      Self::ReadOob => write!(f, "Memory read out of bounds"),
      Self::IllegalOp => write!(f, "Illegal instruction reached"),

      Self::BadCall => {
        write!(f, "Attempted to call function which does not exist")
      },
      Self::NoExec => write!(
        f,
        "No exe instruction found. Are you trying to execute a library?"
      ),
      Self::NoFunc => {
        write!(f, "Cannot begin Recital, no functions have been defined")
      },
      Self::ExecLimit => write!(f, "Execution limit reached, compile and run natively to do really big stuff!"),
      Self::EndReached => write!(f, "Successfully terminated"),
    }
  }
}

pub type CResult = Result<(), Error>;
