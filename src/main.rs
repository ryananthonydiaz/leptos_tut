use leptos::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;

#[component]
fn App() -> impl IntoView {
    let name: ReadSignal<String>;
    let set_name: WriteSignal<String>;
    (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call `HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        
        set_name(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/>})
}
