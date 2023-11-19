use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count)= create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 2);
            }
        >
            "Click me: "
            {count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count.get()}
        </p>

        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            // Note: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once.
            {count()}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
