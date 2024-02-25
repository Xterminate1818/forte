use crate::{OpKind, Word};
use log::*;

/// Assembler

#[derive(Debug, Copy, Clone)]
pub enum AsmError {
  ExpectedNum,
  InvalidOp,
}

fn parse_num(input: &str) -> Result<Word, AsmError> {
  debug!("Parsing {} as number", input);
  if let Ok(n) = input.parse::<Word>() {
    return Ok(n);
  }
  if let Ok(n) = Word::from_str_radix(input.trim_start_matches("0x"), 16) {
    return Ok(n);
  }
  Err(AsmError::ExpectedNum)
}

fn parse_op(input: &str) -> Result<Word, AsmError> {
  debug!("Parsing {} as opcode", input);
  let op = match input {
    "push" => OpKind::Push,
    "pop" => OpKind::Pop,
    "dup" => OpKind::Dup,
    "add" => OpKind::Add,
    "dif" => OpKind::Dif,
    "mul" => OpKind::Mul,
    "div" => OpKind::Div,
    "rem" => OpKind::Rem,
    "and" => OpKind::And,
    "or" => OpKind::Or,
    "xor" => OpKind::Xor,
    "shr" => OpKind::Shr,
    "shl" => OpKind::Shl,
    "beq" => OpKind::Beq,
    "bne" => OpKind::Bne,
    "bgt" => OpKind::Bgt,
    "blt" => OpKind::Blt,
    "fun" => OpKind::Fun,
    "call" => OpKind::Call,
    "ret" => OpKind::Ret,
    "loop" => OpKind::Loop,
    "iter" => OpKind::Iter,
    "exe" => OpKind::Exe,
    "sto" => OpKind::Sto,
    "lod" => OpKind::Lod,
    "len" => OpKind::Len,
    _ => return Err(AsmError::InvalidOp),
  };
  Ok(op as Word)
}

pub fn assemble(input: &str) -> Result<Vec<Word>, AsmError> {
  let mut last: Option<Word> = None;

  let split = input.split_whitespace();
  let mut output = Vec::with_capacity(split.size_hint().0);
  for part in split {
    // If just parsed op, next should be num
    if last.is_some() {
      let num = parse_num(part)?;
      let op = last.take().unwrap();
      let word = (op << 64) | num;
      output.push(word);
    }
    // Otherwise, could be op or num
    else {
      if let Ok(op) = parse_op(part) {
        last = Some(op);
      } else {
        output.push(parse_num(part)?);
      }
    }
  }
  if last.is_some() {
    Err(AsmError::ExpectedNum)
  } else {
    Ok(output)
  }
}
