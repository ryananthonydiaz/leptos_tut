use leptos::*;

#[component]
fn App() -> impl IntoView {
    let value: ReadSignal<i32>;
    let set_value: WriteSignal<i32>;
    (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;

    view! {
        <main>
        {move || match is_odd() {
                                    true if value() == 1 => {
                                        // returns HtmlElement<Pre>
                                        view! { <pre>"One"</pre> }.into_any()
                                    },
                                    false if value() == 2 => {
                                        // returns HtmlElement<P>
                                        view! {
                                            <p>"Two"</p>
                                        }.into_any()
                                    },
                                    _ => view! { <textarea>{value()}</textarea>}.into_any()
                                }}
        </main>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
