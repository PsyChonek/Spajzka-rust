use web_sys::console;
use yew::prelude::*;

use crate::app::{NotesAction, NotesState};

#[derive(PartialEq, Properties)]
pub struct NotesProps {}

#[function_component(Notes)]
pub fn notes(props: &NotesProps) -> Html {
    let notes = use_context::<UseReducerHandle<NotesState>>()
        .expect("No NotesState context found!");
    let note_text = use_state(|| "".to_string());

    // Handler for textarea input
    let on_input = {
        let note_text = note_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                note_text.set(input.value());
                console::log_1(&format!("Updated note_text: {}", input.value()).into());
            }
        })
    };

    let on_add_note: Callback<MouseEvent> = {
        let notes = notes.clone();
        let note_text = note_text.clone();
        Callback::from(move |_: MouseEvent| {
            console::log_1(&format!("Adding note: {}", *note_text).into());

            if !note_text.is_empty() {
                console::log_1(&format!("Adding note: {}", *note_text).into());
                notes.dispatch(NotesAction::AddNote((*note_text).clone()));
                note_text.set("".to_string());
            }
        })
    };

    let NotesProps {} = props;
    html! {
        <div>
            <h2>{ "Notes" }</h2>
            <div class="notes">
                <textarea value={(*note_text).clone()} oninput={on_input}></textarea>
                <button onclick={on_add_note}>{ "Add Note" }</button>
            </div>
        </div>
    }
}
