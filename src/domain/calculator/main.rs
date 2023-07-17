use itertools::Itertools;
use std::{
  fmt, ops,
  sync::{Arc, Mutex},
};

use crate::application::logger::console;

#[derive(Clone)]
pub struct Calculator {
  stack: Arc<Mutex<Vec<Entry>>>,
}

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

impl Calculator {
  pub fn new() -> Calculator {
    let stack = Arc::new(Mutex::new(vec![]));
    Calculator { stack }
  }

  pub fn entry(&mut self, arg: Entry) {
    let mut stack = self.stack.lock().unwrap();
    let len = stack.len();
    if len > 0 {
      let last = &stack[stack.len() - 1];
      match arg {
        Entry::Add | Entry::Sub | Entry::Mul | Entry::Div | Entry::Rem => match last {
          &Entry::Add | &Entry::Sub | &Entry::Mul | &Entry::Div | &Entry::Rem => {
            stack.pop();
            stack.push(arg);
          }
          _ => {
            stack.push(arg);
          }
        },
        Entry::Call => match last {
          &Entry::Return | &Entry::Num(_) => {
            stack.push(Entry::Mul);
            stack.push(arg);
          }
          _ => {
            stack.push(arg);
          }
        },
        Entry::Return => match last {
          &Entry::Call => {}
          _ => {
            stack.push(arg);
          }
        },
        Entry::Num(_) => match last {
          &Entry::Return | &Entry::Num(_) => {
            stack.push(Entry::Mul);
            stack.push(arg);
          }
          _ => {
            stack.push(arg);
          }
        },
      };
    } else {
      match arg {
        Entry::Call | Entry::Num(_) => stack.push(arg),
        _ => {}
      }
    };
    console::log(
      "calculator",
      format!("formula = \"{}\"", Itertools::join(&mut stack.iter(), " ")),
    );
  }

  pub fn get_formula(&self) -> String {
    let stack = self.stack.lock().unwrap();
    Itertools::join(&mut stack.iter(), " ")
  }

  pub fn get_result(&self) -> Option<f32> {
    let stack = self.stack.lock().unwrap();
    calc(&stack)
  }

  pub fn clear_last_entry(&mut self) -> Option<Entry> {
    let mut stack = self.stack.lock().unwrap();
    stack.pop()
  }

  pub fn clear(&mut self) {
    let mut stack = self.stack.lock().unwrap();
    stack.clear();
  }
}

impl ops::Deref for Calculator {
  type Target = Arc<Mutex<Vec<Entry>>>;

  fn deref(&self) -> &Self::Target {
    &self.stack
  }
}

impl ops::DerefMut for Calculator {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.stack
  }
}

fn calc(args: &Vec<Entry>) -> Option<f32> {
  let mut args = args.clone();
  if args.len() == 0 {
    return None;
  }
  if args.len() == 1 {
    if let Entry::Num(num) = args[0] {
      return Option::from(num);
    } else {
      return None;
    }
  }
  Option::from(0.0)
}
