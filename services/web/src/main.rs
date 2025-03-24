mod app;
mod counter;   
mod notes;
mod notes_view;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
