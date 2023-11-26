use leptos::*;

#[component]
fn App() -> impl IntoView {
    let value: ReadSignal<i32>;
    let set_value: WriteSignal<i32>;
    (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    let message = move ||  is_odd().then(|| "Ding ding ding!");

    view! {
        <p>{message}</p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
