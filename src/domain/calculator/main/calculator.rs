use crate::application::logger::console;
use itertools::Itertools;

use super::Entry;

pub fn calc(args: &Vec<Entry>) -> Result<f32, String> {
  console::log(
    "calculator",
    format!(
      "calc(args = \"{}\")",
      Itertools::join(&mut args.clone().iter(), ""),
    ),
  );
  if args.len() == 0 {
    return Ok(0.0);
  }
  if args.len() == 1 {
    if let Entry::Num(num) = args[0] {
      return Ok(num);
    } else {
      return Err(format!("Expected numeric, but found `{}`", args[0]));
    }
  }

  let mut args = args.clone();

  args = unwrap_parentheses(&args)?;
  args = calculate_multiplication_division_remainder(&args)?;
  args = calculate_addition_subtraction(&args)?;

  if args.len() != 1 {
    return Err("something wrong".to_string());
  }
  if let Entry::Num(num) = args[0] {
    return Ok(num);
  } else {
    return Err("something wrong".to_string());
  }
}

pub fn unwrap_parentheses(args: &Vec<Entry>) -> Result<Vec<Entry>, String> {
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
          let sub_result = calc(&args[call + 1..i].to_vec())?;
          let original = args.clone();
          args = original[0..call].to_vec();
          args.push(Entry::Num(sub_result));
          args.append(&mut original[i + 1..].to_vec());
          i = call;
          break 'call;
        }
        if i == args.len() - 1 {
          return Err("`)` not found".to_string());
        }
      }
    }
    if let Entry::Return = args[i] {
      return Err("`(` not found".to_string());
    }
  }
  Ok(args)
}

pub fn calculate_multiplication_division_remainder(
  args: &Vec<Entry>,
) -> Result<Vec<Entry>, String> {
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
          return Err("Expected numeric, but found `EOF`".to_string());
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
                return Err("Expected operator ('*', '/', '%'), but found `EOF`".to_string());
              }
            }
            if original.len() > i + 2 {
              args.append(&mut original[i + 2..].to_vec());
              i -= 1;
            }
          } else {
            return Err(format!("Expected numeric, but found `{}`", args[i + 1]));
          }
        } else {
          return Err(format!("Expected numeric, but found `{}`", args[i - 1]));
        }
      }
      _ => {}
    }
  }
  Ok(args)
}

fn calculate_addition_subtraction(args: &Vec<Entry>) -> Result<Vec<Entry>, String> {
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
          return Err("Expected numeric, but found `EOF`".to_string());
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
                return Err("Expected operator ('+', '-'), but found `EOF`".to_string());
              }
            }
            if original.len() > i + 2 {
              args.append(&mut original[i + 2..].to_vec());
              i -= 1;
            }
          } else {
            return Err(format!("Expected numeric, but found `{}`", args[i + 1]));
          }
        } else {
          return Err(format!("Expected numeric, but found `{}`", args[i - 1]));
        }
      }
      _ => {}
    }
  }
  Ok(args)
}
