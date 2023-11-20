use leptos::*;

#[component]
fn App() -> impl IntoView {
    let name: ReadSignal<String>;
    let set_name: WriteSignal<String>;
    (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <input
            type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }
            
            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name

        />
        <p>"Name is: " {name}</p> 
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
