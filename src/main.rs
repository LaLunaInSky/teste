use leptos::{
    prelude::*,
    mount::mount_to_body
};

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>
                "Teste"
            </h1>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}