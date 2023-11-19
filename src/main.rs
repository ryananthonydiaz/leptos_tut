use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count)= create_signal(0);
    let double_count = move || count() * 2;


    view! {
        <button
            class=("red", move || count() % 2 == 1)
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
        <progress
            max="50"
            // signals are functions, so this <=> `move || count.get()`
            value=double_count
        />
        <p>
            "Double Counts: "
            // and again here
            {double_count}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
