mod application;
mod domain;

use domain::calculator::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
  let calc = use_state(|| Calculator::new());
  let ans = use_state(|| "".to_string());
  let formula = use_state(|| "".to_string());
  let num_entry = use_state(|| "".to_string());

  let onclick = {
    let calc = calc.clone();
    let formula = formula.clone();
    let num_entry = num_entry.clone();
    let ans = ans.clone();
    Callback::from(move |e: MouseEvent| {
      e.prevent_default();
      let mut calc = (*calc).clone();
      let input = e.target_dyn_into::<HtmlInputElement>();
      if let Some(input) = input {
        let value = input.value();
        match value.as_str() {
          "(" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Call);
          }
          ")" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Return);
          }
          "%" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Rem);
          }
          "/" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Div);
          }
          "*" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Mul);
          }
          "-" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
              calc.entry(Entry::Sub);
            } else {
              num_entry.set((*num_entry).to_string() + value.as_str());
            }
          }
          "+" => {
            if (*num_entry).len() != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            calc.entry(Entry::Add);
          }
          "=" => {
            let len = (*num_entry).len();
            if len != 0 {
              calc.entry(Entry::Num(num_entry.parse().unwrap()));
              num_entry.set("".to_string());
            }
            let result = calc.get_result();
            if result.is_ok() {
              let result = result.unwrap();
              ans.set(format!("Ans = {}", result));
              num_entry.set(format!("{}", result));
              calc.clear();
            } else {
              ans.set("Error".to_string());
            }
          }
          "CE" => {
            let len = (*num_entry).len();
            if len != 0 {
              let mut s = (*num_entry).to_string();
              s.pop();
              num_entry.set(s);
            } else {
              let entry = calc.clear_last_entry();
              if let Some(e) = entry {
                if let Entry::Num(num) = e {
                  num_entry.set(format!("{}", num))
                } else {
                  let entry = calc.clear_last_entry();
                  if let Some(e) = entry {
                    if let Entry::Num(num) = e {
                      num_entry.set(format!("{}", num))
                    } else {
                      calc.entry(e);
                    }
                  }
                }
              }
            }
          }
          _ => {
            num_entry.set((*num_entry).to_string() + value.as_str());
          }
        }
      }

      formula.set(calc.get_formula());
    })
  };

  html! {
    <div class={classes!("p-8")}>
      <div class={classes!("outline","outline-1","rounded","max-w-[480px]")}>
        <div class={classes!("pt-2", "pr-4", "text-right")}>
          <div class={classes!("h-6")}>{(*ans).to_string()}</div>
          <div class={classes!("h-8","text-2xl","text-neutral")}>{(*formula).to_string()}{" "}{(*num_entry).to_string()}</div>
        </div>
        <div class={classes!("divider","my-0")} />
        <div class={classes!(
          "grid","grid-cols-4","gap-1","p-2",
          "child:btn",
        )}>
          <input type={"button"} value={"("} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={")"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"%"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"CE"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"7"} onclick={&onclick} />
          <input type={"button"} value={"8"} onclick={&onclick} />
          <input type={"button"} value={"9"} onclick={&onclick} />
          <input type={"button"} value={"/"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"4"} onclick={&onclick} />
          <input type={"button"} value={"5"} onclick={&onclick} />
          <input type={"button"} value={"6"} onclick={&onclick} />
          <input type={"button"} value={"*"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"1"} onclick={&onclick} />
          <input type={"button"} value={"2"} onclick={&onclick} />
          <input type={"button"} value={"3"} onclick={&onclick} />
          <input type={"button"} value={"-"} onclick={&onclick} class={classes!("!btn-neutral")} />
          <input type={"button"} value={"0"} onclick={&onclick} />
          <input type={"button"} value={"."} onclick={&onclick} />
          <input type={"button"} value={"="} onclick={&onclick} class={classes!("!btn-primary")} />
          <input type={"button"} value={"+"} onclick={&onclick} class={classes!("!btn-neutral")} />
        </div>
      </div>
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
