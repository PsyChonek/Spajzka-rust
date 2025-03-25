use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CounterProps {
    // Initial value
    pub init: i32,
}

#[function_component(Counter)]
pub fn counter(props: &CounterProps) -> Html {
    // State for the counter
    let count = use_state(|| props.init);

    html! {
        <div>
            // Count display
            <p>{ "Count: " } { *count }</p>

            <div class="buttons">
                // Increment button
                <button onclick={
                    let count = count.clone();
                    move |_| count.set(*count + 1)
                }>
                    { "Increment" }
                </button>

                // Decrement button
                <button onclick={
                    let count = count.clone();
                    move |_| count.set(*count - 1)
                }>
                    { "Decrement" }
                </button>
            </div>
       </div>
    }
}
