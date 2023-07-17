mod application;

use yew::prelude::*;
use application::logger::console;

#[function_component]
fn App() -> Html {
  console::log("main", "loaded".to_string());

  html! {
    <div class={classes!("p-8")}>
      <div class={classes!("outline","outline-1","rounded","max-w-[480px]")}>
        <div class={classes!("pt-2", "pr-4", "text-right")}>
          <div>{"Ans = 10"}</div>
          <div class={classes!("text-2xl","text-neutral")}>{"10  + 10 + 10"}</div>
        </div>
        <div class={classes!("divider","my-0")} />
        <div class={classes!(
          "grid","grid-cols-4","gap-1","p-2",
          "child:btn",
        )}>
          <button class={classes!("!btn-neutral")}>{"("}</button>
          <button class={classes!("!btn-neutral")}>{")"}</button>
          <button class={classes!("!btn-neutral")}>{"%"}</button>
          <button class={classes!("!btn-neutral")}>{"CE"}</button>
          <button>{"7"}</button>
          <button>{"8"}</button>
          <button>{"9"}</button>
          <button class={classes!("!btn-neutral")}>{"/"}</button>
          <button>{"4"}</button>
          <button>{"5"}</button>
          <button>{"6"}</button>
          <button class={classes!("!btn-neutral")}>{"*"}</button>
          <button>{"1"}</button>
          <button>{"2"}</button>
          <button>{"3"}</button>
          <button class={classes!("!btn-neutral")}>{"-"}</button>
          <button>{"0"}</button>
          <button>{"."}</button>
          <button class={classes!("!btn-primary")}>{"="}</button>
          <button class={classes!("!btn-neutral")}>{"+"}</button>
        </div>
      </div>
    </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
