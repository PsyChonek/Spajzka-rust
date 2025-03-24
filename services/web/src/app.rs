use yew::prelude::*;

use crate::{counter::Counter, notes::Notes, notes_view::NotesView};

#[derive(Clone, PartialEq)]
pub struct NotesState {
    pub notes: Vec<(i32, String)>,
}

impl Default for NotesState {
    fn default() -> Self {
        Self { notes: vec![] }
    }
}

pub enum NotesAction {
    AddNote(String),
    RemoveNote(i32),
    UpdateNote { index: i32, text: String },
}

impl Reducible for NotesState {
    type Action = NotesAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            NotesAction::AddNote(text) => {
                let mut new_notes = self.notes.clone();
                let index = new_notes.len() as i32; // Use the current length as the index
                new_notes.push((index, text));
                std::rc::Rc::new(Self { notes: new_notes })
            }
            NotesAction::RemoveNote(index) => {
                let new_notes = self
                    .notes
                    .iter()
                    .cloned()
                    .filter(|(i, _)| *i != index)
                    .collect();
                std::rc::Rc::new(Self { notes: new_notes })
            }
            NotesAction::UpdateNote { index, text } => {
                let mut new_notes = self.notes.clone();
                if let Some(note) = new_notes.iter_mut().find(|(i, _)| *i == index) {
                    note.1 = text;
                }
                std::rc::Rc::new(Self { notes: new_notes })
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let notes: UseReducerHandle<NotesState> = use_reducer(NotesState::default);

    html! {
        <ContextProvider<UseReducerHandle<NotesState>> context={notes}>
            <main>
                <h1>{ "Hello World!" }</h1>
                <section class="content">
                    <h2>{ "Counter" }</h2>
                    <Counter init=0 />
                    <Notes />
                    <NotesView />
                </section>
            </main>
        </ContextProvider<UseReducerHandle<NotesState>>>
    }
}
