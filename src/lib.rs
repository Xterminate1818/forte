mod asm;
mod op;
mod util;

use op::*;
use util::*;

pub type Byte = u32;
pub type Word = u128;

#[derive(Debug, Clone, Copy)]
pub enum Phase {
  Warmup,
  Recital,
}

pub struct Machine {
  ip: Word,
  ebp: Word,
  phase: Phase,
  pub log: String,
  pub cstack: Vec<Word>,
  pub pstack: Vec<Word>,
  pub fstack: Vec<Word>,
  pub instructions: Vec<Word>,
  pub mem: Vec<Word>,
}

impl Machine {
  pub fn new() -> Self {
    Self {
      ip: 0,
      ebp: 0,
      phase: Phase::Warmup,
      log: "".to_string(),
      cstack: vec![],
      pstack: vec![],
      fstack: vec![],
      instructions: vec![],
      mem: vec![0; 1024],
    }
  }

  pub fn read(&mut self, src: Vec<u128>) {
    self.instructions = src;
  }

  fn memset(&mut self, address: Word, val: Word) -> CResult {
    let address: usize = match address.try_into() {
      Ok(a) => a,
      Err(_) => return Err(Error::WriteOob),
    };
    let reference = self.mem.get_mut(address).ok_or(Error::WriteOob)?;
    *reference = val;
    Ok(())
  }

  fn memget(&mut self, address: Word) -> Result<Word, Error> {
    let address: usize = match address.try_into() {
      Ok(a) => a,
      Err(_) => return Err(Error::WriteOob),
    };
    self.mem.get(address).ok_or(Error::WriteOob).copied()
  }

  fn c_pop(&mut self) -> Result<Word, Error> {
    match self.cstack.pop() {
      Some(i) => Ok(i),
      None => Err(Error::StackUnderflow),
    }
  }

  fn p_pop(&mut self) -> Result<Word, Error> {
    match self.pstack.pop() {
      Some(i) => Ok(i),
      None => Err(Error::StackUnderflow),
    }
  }

  fn p_peak(&mut self) -> Result<Word, Error> {
    match self.pstack.last() {
      Some(i) => Ok(*i),
      None => Err(Error::StackUnderflow),
    }
  }

  fn call(&mut self, address: Word) -> CResult {
    let address: usize = address.try_into().map_err(|_| Error::BadCall)?;
    let address = *self.fstack.get(address).ok_or(Error::BadCall)?;

    self.cstack.push(self.ip);
    self.cstack.push(self.cstack.len() as Word);
    if address >= self.instructions.len() as Word {
      return Err(Error::BadCall);
    }
    self.ip = address;
    Ok(())
  }

  fn _return(&mut self) -> CResult {
    let ebp = self.c_pop()?;
    let ip = self.c_pop()?;
    self.ip = ip;
    self.ebp = ebp;
    Ok(())
  }

  pub fn step(&mut self) -> CResult {
    let word = self
      .instructions
      .get(self.ip as usize)
      .ok_or(Error::NoExec)?;
    let instr: Op = match word.try_into() {
      Ok(i) => i,
      Err(_) => return Err(Error::IllegalOp),
    };
    self.log.push_str(&format!(
      "[{:?}] {}: {:?} {}\n",
      self.phase, self.ip, instr.kind, instr.var,
    ));
    self.ip += 1;
    match self.phase {
      Phase::Warmup => self.step_warmup(instr),
      Phase::Recital => self.step_recital(instr),
    }
  }

  pub fn step_recital(&mut self, instr: Op) -> CResult {
    match instr.kind {
      OpKind::Push => {
        for _ in 0..instr.var {
          let num = match self.instructions.get(self.ip as usize) {
            Some(n) => n,
            None => return Err(Error::StackUnderflow),
          };
          self.log.push_str(&format!(
            "[Recital] {}: {} added to stack\n",
            self.ip, num
          ));
          self.pstack.push(*num);
          self.ip += 1;
        }
      },
      OpKind::Pop => {
        for _ in 0..instr.var {
          self.p_pop()?;
        }
      },
      OpKind::Dup => {
        let d = self.p_peak().unwrap_or(0);
        for _ in 0..instr.var {
          self.pstack.push(d);
        }
      },
      OpKind::Add => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(a.saturating_add(b));
        }
      },
      OpKind::Dif => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(abs_sub(a, b));
        }
      },
      OpKind::Mul => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(a.saturating_mul(b));
        }
      },
      OpKind::Div => {
        if instr.var == 0 {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(b.checked_div(a).unwrap_or(Word::MAX));
        } else {
          let numerator = self.p_pop()?;
          self.pstack.push(
            numerator
              .checked_div(instr.var as Word)
              .unwrap_or(Word::MAX),
          )
        }
      },
      OpKind::Rem => {
        if instr.var == 0 {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self
            .pstack
            .push(b.checked_rem_euclid(a).unwrap_or(Word::MAX));
        } else {
          let numerator = self.p_pop()?;
          self.pstack.push(
            numerator
              .checked_rem_euclid(instr.var as Word)
              .unwrap_or(Word::MAX),
          )
        }
      },
      OpKind::And => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(a & b);
        }
      },
      OpKind::Or => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(a | b);
        }
      },
      OpKind::Xor => {
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(a ^ b);
        }
      },
      OpKind::Shl => {
        if instr.var == 0 {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(b << a);
        } else {
          let top = self.p_pop()?;
          self.pstack.push(top << instr.var as Word);
        }
      },
      OpKind::Shr => {
        if instr.var == 0 {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          self.pstack.push(b >> a);
        } else {
          let top = self.p_pop()?;
          self.pstack.push(top >> instr.var as Word);
        }
      },
      OpKind::Beq => {
        let mut should_branch = false;
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          should_branch |= a != b;
          self.pstack.push(a);
        }
        // Remove extraneous
        self.p_pop()?;
        if should_branch {
          self.ip += 1;
          self.call(instr.var as Word)?;
        }
      },
      OpKind::Bne => {
        let mut should_branch = false;
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          should_branch |= a == b;
          self.pstack.push(a);
        }
        // Remove extraneous
        self.p_pop()?;
        if should_branch {
          self.ip += 1;
          self.call(instr.var as Word)?;
        }
      },

      OpKind::Bgt => {
        let mut should_branch = false;
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          should_branch |= a <= b;
          self.pstack.push(a);
        }
        // Remove extraneous
        self.p_pop()?;
        if should_branch {
          self.ip += 1;
          self.call(instr.var as Word)?;
        }
      },

      OpKind::Blt => {
        let mut should_branch = false;
        for _ in 0..instr.var {
          let a = self.p_pop()?;
          let b = self.p_pop()?;
          should_branch |= a <= b;
          self.pstack.push(a);
        }
        // Remove extraneous
        self.p_pop()?;
        if should_branch {
          self.ip += 1;
          self.call(instr.var as Word)?;
        }
      },
      OpKind::Fun => {
        // Ignore during execution
      },
      OpKind::Call => {
        self.call(instr.var as Word)?;
      },
      OpKind::Ret => {
        self._return()?;
      },

      OpKind::Loop => {
        self.cstack.push(self.ip);
        self.cstack.push(instr.var as Word);
      },

      OpKind::Iter => {
        let counter = self.c_pop()?;
        let address = self.c_pop()?;
        if counter.saturating_sub(1) != 0 {
          self.cstack.push(address);
          self.cstack.push(counter - 1);
        }
      },
      OpKind::Exe => {
        // Successfully reached end of main!
        self.phase = Phase::Recital;
        return Err(Error::EndReached);
      },

      OpKind::Sto => {
        let word = self.p_pop()?;
        self.memset(instr.var as Word, word)?;
      },

      OpKind::Lod => {
        let word = self.memget(instr.var as Word)?;
        self.pstack.push(word);
      },
      OpKind::Len => {
        self.pstack.push(self.pstack.len() as Word - self.ebp);
      },
      _ => {},
    };

    self
      .log
      .push_str(&format!("[STACK]:     {:?}\n", self.pstack));
    Ok(())
  }

  pub fn step_warmup(&mut self, instr: Op) -> CResult {
    match instr.kind {
      OpKind::Fun => self.fstack.push(self.ip),
      OpKind::Exe => {
        self.phase = Phase::Recital;
        self.ip = match self.fstack.last() {
          Some(i) => *i,
          None => return Err(Error::StackUnderflow),
        }
      },
      OpKind::Push => {
        for _ in 0..instr.var {
          self.ip += 1;
        }
      },
      _ => {},
    }
    Ok(())
  }

  pub fn steps(&mut self, n: usize) -> CResult {
    for _ in 0..n {
      self.step()?;
    }
    Ok(())
  }

  pub fn run(&mut self) -> String {
    loop {
      match self.step() {
        Ok(_) => {},
        Err(e) => {
          if e == Error::EndReached {
            self
              .log
              .push_str(&format!("[BRAVO] Final stack: {:?}\n", self.pstack))
          } else {
            self.log.push_str(&format!("[CLAM] {}\n", e))
          }
          return self.log.clone();
        },
      }
    }
  }
}

pub fn interpret(program: &str) -> String {
  let mut m = Machine::new();
  let program = match asm::assemble(program) {
    Ok(p) => p,
    Err(e) => return format!("{}", e),
  };
  m.read(program);
  let s = m.run();
  s
}

fn main() {
  let p = include_str!("../demo.asm");
  let s = interpret(&p);
  println!("{s}");
}
