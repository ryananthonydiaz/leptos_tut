use leptos::*;

#[component]
fn ProgressBar(
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/>
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let count: ReadSignal<i32>;
    let set_count: WriteSignal<i32>;
    (count, set_count) = create_signal(0);
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
        <ProgressBar progress=count />
        // add a second progress bar
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
