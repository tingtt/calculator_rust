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
  console::log(
    "calculator",
    format!(
      "calc(args = \"{}\")",
      Itertools::join(&mut args.clone().iter(), ""),
    ),
  );
  if args.len() == 0 {
    return None;
  }
  if args.len() == 1 {
    if let Entry::Num(num) = args[0] {
      return Option::from(num);
    } else {
      console::log(
        "calculator",
        format!("Error: Expected numeric, but found `{}`", args[0]),
      );
      return None;
    }
  }

  let mut args = args.clone();

  if let Some(args_) = unwrap_parentheses(&args) {
    args = args_;
  } else {
    return None;
  }
  if let Some(args_) = calculate_multiplication_division_remainder(&args) {
    args = args_;
  } else {
    return None;
  }
  if let Some(args_) = calculate_addition_subtraction(&args) {
    args = args_;
  } else {
    return None;
  }

  if args.len() != 1 {
    console::log("calculator", "Error: something wrong".to_string());
    return None;
  }
  if let Entry::Num(num) = args[0] {
    return Option::from(num);
  } else {
    console::log("calculator", "Error: something wrong".to_string());
    return None;
  }
}

fn unwrap_parentheses(args: &Vec<Entry>) -> Option<Vec<Entry>> {
  let mut args = args.clone();
  for mut i in 0..args.len() {
    if i > args.len() - 1 {
      break;
    }
    if let Entry::Call = args[i] {
      let call = i;
      let mut call_found_cound = 1;
      'call: loop {
        i += 1;
        if let Entry::Call = args[i] {
          call_found_cound += 1;
        }
        if let Entry::Return = args[i] {
          call_found_cound -= 1;
          if call_found_cound > 0 {
            continue 'call;
          }
          let sub_result = calc(&args[call + 1..i].to_vec());
          if let Some(sub_result) = sub_result {
            let original = args.clone();
            args = original[0..call].to_vec();
            args.push(Entry::Num(sub_result));
            args.append(&mut original[i + 1..].to_vec());
            i = call;
            break 'call;
          } else {
            console::log("calculator", "Error: something wrong".to_string());
            return None;
          }
        }
        if i == args.len() - 1 {
          console::log("calculator", "Error: `)` not found".to_string());
          return None;
        }
      }
    }
    if let Entry::Return = args[i] {
      console::log("calculator", "Error: `(` not found".to_string());
      return None;
    }
  }
  Option::from(args)
}

fn calculate_multiplication_division_remainder(args: &Vec<Entry>) -> Option<Vec<Entry>> {
  let mut args = args.clone();
  let mut i = 0;
  loop {
    i += 1;
    if i > args.len() - 1 {
      break;
    }
    match args[i] {
      Entry::Mul | Entry::Div | Entry::Rem => {
        if i == args.len() - 1 {
          console::log(
            "calculator",
            "Error: Expected numeric, but found `EOF`".to_string(),
          );
          return None;
        }
        console::log(
          "calculator",
          format!(
            "args = \"{}\", cursor = {}",
            Itertools::join(&mut args.clone().iter(), ""),
            i
          ),
        );
        if let Entry::Num(num1) = args[i - 1] {
          if let Entry::Num(num2) = args[i + 1] {
            let original = args.clone();
            args = original[0..i - 1].to_vec();
            match original[i] {
              Entry::Mul => args.push(Entry::Num(num1 * num2)),
              Entry::Div => args.push(Entry::Num(num1 / num2)),
              Entry::Rem => args.push(Entry::Num(num1 % num2)),
              _ => {
                return None;
              }
            }
            if original.len() > i + 2 {
              args.append(&mut original[i + 2..].to_vec());
              i -= 1;
            }
          } else {
            console::log(
              "calculator",
              format!("Error: Expected numeric, but found `{}`", args[i + 1]),
            );
            return None;
          }
        } else {
          console::log(
            "calculator",
            format!("Error: Expected numeric, but found `{}`", args[i - 1]),
          );
          return None;
        }
      }
      _ => {}
    }
  }
  Option::from(args)
}

fn calculate_addition_subtraction(args: &Vec<Entry>) -> Option<Vec<Entry>> {
  let mut args = args.clone();
  let mut i = 0;
  loop {
    i += 1;
    if i > args.len() - 1 {
      break;
    }
    match args[i] {
      Entry::Add | Entry::Sub => {
        if i == args.len() - 1 {
          return None;
        }
        console::log(
          "calculator",
          format!(
            "args = \"{}\", cursor = {}",
            Itertools::join(&mut args.clone().iter(), ""),
            i
          ),
        );
        if let Entry::Num(num1) = args[i - 1] {
          if let Entry::Num(num2) = args[i + 1] {
            let original = args.clone();
            args = original[0..i - 1].to_vec();
            match original[i] {
              Entry::Add => args.push(Entry::Num(num1 + num2)),
              Entry::Sub => args.push(Entry::Num(num1 - num2)),
              _ => {
                return None;
              }
            }
            if original.len() > i + 2 {
              args.append(&mut original[i + 2..].to_vec());
              i -= 1;
            }
          } else {
            console::log(
              "calculator",
              format!("Error: Expected numeric, but found `{}`", args[i + 1]),
            );
            return None;
          }
        } else {
          console::log(
            "calculator",
            format!("Error: Expected numeric, but found `{}`", args[i - 1]),
          );
          return None;
        }
      }
      _ => {}
    }
  }
  Option::from(args)
}
