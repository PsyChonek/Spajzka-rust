use yew::prelude::*;

use crate::app::NotesState;

#[derive(PartialEq, Properties)]
pub struct NotesViewProps {}

#[function_component(NotesView)]
pub fn notes_view(props: &NotesViewProps) -> Html {
    let notes = use_context::<UseReducerHandle<NotesState>>()
        .expect("No NotesState context found!");
    
    html! {
        <div class="notes-list">
        { for notes.notes.iter().map(|(index, note)| {
            html! {
                <div class="note">
                    <h4>{ index }</h4>
                    <p>{ note }</p>
                </div>
            }
        }) }
        </div>
    }
}
