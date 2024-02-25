use crate::{OpKind, Word};

/// Assembler

#[derive(Debug, Copy, Clone)]
pub enum AsmError {
  ExpectedNum,
  InvalidOp,
}

#[derive(Debug, Clone)]
pub struct TaggedAsmError {
  kind: AsmError,
  word: String,
}

impl std::fmt::Display for TaggedAsmError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[Assembler] Error parsing token: ")?;
    match self.kind {
      AsmError::ExpectedNum => {
        write!(f, "Expected number, found {}", self.word)
      },
      AsmError::InvalidOp => write!(f, "Expected opcode, found {}", self.word),
    }
  }
}

fn parse_num(input: &str) -> Result<Word, AsmError> {
  if let Ok(n) = input.parse::<Word>() {
    return Ok(n);
  }
  if let Ok(n) = Word::from_str_radix(input.trim_start_matches("0x"), 16) {
    return Ok(n);
  }
  Err(AsmError::ExpectedNum)
}

fn parse_op(input: &str) -> Result<Word, AsmError> {
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

pub fn assemble(input: &str) -> Result<Vec<Word>, TaggedAsmError> {
  let mut last: Option<Word> = None;

  let split = input.split_whitespace();
  let mut output = Vec::with_capacity(split.size_hint().0);
  for part in split {
    // If just parsed op, next should be num
    if last.is_some() {
      let num = parse_num(part).map_err(|kind| TaggedAsmError {
        kind,
        word: part.to_string(),
      })?;
      let op = last.take().unwrap();
      let word = (op << 64) | num;
      output.push(word);
    }
    // Otherwise, could be op or num
    else {
      if let Ok(op) = parse_op(part) {
        last = Some(op);
      } else {
        output.push(parse_num(part).map_err(|kind| TaggedAsmError {
          kind,
          word: part.to_string(),
        })?);
      }
    }
  }
  if let Some(l) = last {
    Err(TaggedAsmError {
      kind: AsmError::ExpectedNum,
      word: l.to_string(),
    })
  } else {
    Ok(output)
  }
}
