use yew::prelude::*;
use yew_hooks::prelude::*;

/// Home page
#[function_component(About)]
pub fn about() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())

    };

    html! {
        <div class="app">     
          <div class="app-header">       
                <p>
                    <button onclick={ondecrease}>{ "Decrease" }</button>
                    <p>{ *counter }</p>
                    <button onclick={onincrease}>{ "Increase" }</button>
                </p>
          </div>
        </div>
    }
}
