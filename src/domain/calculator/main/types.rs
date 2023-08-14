use std::fmt;

#[derive(Clone)]
pub enum Entry {
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Call,
  Return,
  Num(f32),
}

impl Entry {
  fn as_str(&self) -> String {
    match self {
      Entry::Add => String::from("+"),
      Entry::Sub => String::from("-"),
      Entry::Mul => String::from("*"),
      Entry::Div => String::from("/"),
      Entry::Rem => String::from("%"),
      Entry::Call => String::from("("),
      Entry::Return => String::from(")"),
      Entry::Num(i) => format!("{}", i),
    }
  }
}

impl fmt::Display for Entry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
