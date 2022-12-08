use gloo::console::log;
use serde::de::value;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}


#[function_component(Name)]
pub fn name(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(value);
    });

    let value = props.name.clone();

    html! {
      <div class="app">
        <div class="app-header">
          <p>{ "Enter your name:" }</p>
          <p>{"check the terminal for your answer"}</p>
          <input type="text" name={props.name.clone()} onchange={onchange} />
          //print the name
          {value}
        </div>
      </div>
    }
}
