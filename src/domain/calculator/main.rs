use crate::application::logger::console;
use itertools::Itertools;
use std::{
  ops,
  sync::{Arc, Mutex},
};

pub mod types;
pub use types::Entry;
mod calculator;

#[derive(Clone)]
pub struct Calculator {
  stack: Arc<Mutex<Vec<Entry>>>,
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

  pub fn get_result(&self) -> Result<f32, String> {
    let stack = self.stack.lock().unwrap();
    calculator::calc(&stack)
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
