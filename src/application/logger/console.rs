use std::panic::Location;
use web_sys::console;

#[track_caller]
pub fn log(group: &str, arg: String) {
  console::log_3(
    &format!("[wasm-rust/{}]", group).into(),
    &arg.into(),
    &format!("({})", Location::caller()).into(),
  )
}
