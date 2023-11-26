use leptos::*;

#[component]
fn App() -> impl IntoView {
    let value: ReadSignal<i32>;
    let set_value: WriteSignal<i32>;
    (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    let message = move || match value() {
        0 => "Zero",
        1 => "One",
        n if is_odd() => "Odd",
        _ => "Even",
    };

    view! {
        <p>{message}</p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
