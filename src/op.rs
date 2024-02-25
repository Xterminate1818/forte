use crate::Word;

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum OpKind {
  // P-Stack
  Push,
  Pop,
  Dup,
  // Math
  Add,
  Dif,
  Mul,
  Div,
  Rem,
  // Bool
  And,
  Or,
  Xor,
  Shr,
  Shl,
  // C-Stack
  Beq,
  Bne,
  Bgt,
  Blt,
  // I-Stack
  /// Begin function def
  Fun,
  Call,
  Ret,
  /// Begin subroutine
  Loop,
  /// Iterate the loop
  Iter,
  /// Break loop early
  Exe,
  // Memory
  Sto,
  Lod,
  // Misc
  // Length of the stack currently
  Len,

  LAST,
}

impl TryFrom<u64> for OpKind {
  type Error = ();

  fn try_from(value: u64) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => Self::Push,
      1 => Self::Pop,
      2 => Self::Dup,
      3 => Self::Add,
      4 => Self::Dif,
      5 => Self::Mul,
      6 => Self::Div,
      7 => Self::Rem,
      8 => Self::And,
      9 => Self::Or,
      10 => Self::Xor,
      11 => Self::Shr,
      12 => Self::Shl,
      13 => Self::Beq,
      14 => Self::Bne,
      15 => Self::Bgt,
      16 => Self::Blt,
      17 => Self::Fun,
      18 => Self::Call,
      19 => Self::Ret,
      20 => Self::Loop,
      21 => Self::Iter,
      22 => Self::Exe,
      23 => Self::Sto,
      24 => Self::Lod,
      25 => Self::Len,
      _ => return Err(()),
    })
  }
}

#[derive(Clone, Debug)]
pub struct Op {
  pub op: u64,
  pub kind: OpKind,
  pub var: u64,
}

impl Op {
  pub fn new(kind: OpKind, var: u64) -> Word {
    let top = (kind as u64 as u128) << 64;
    let bottom = var as u128;
    top | bottom
  }
}

impl TryFrom<&u128> for Op {
  type Error = ();

  fn try_from(value: &u128) -> Result<Self, Self::Error> {
    let top = value >> 64;
    let bottom = value & 0xFFFF_FFFF;
    let kind: OpKind = (top as u64).try_into()?;
    Ok(Self {
      kind,
      op: top as u64,
      var: bottom as u64,
    })
  }
}
