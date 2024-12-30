use sycamore::prelude::*;

#[component(inline_props)]
fn Wrapper(children: Children) -> View {
    view! {
        div {
            (children)
        }
    }
}

#[derive(Props)]
struct HelloProps {
    name: String,
}

#[component]
fn Hello(props: HelloProps) -> View {
    view! {
        p { "Hello, " (props.name) "!" }
    }
}

#[component]
fn HelloWorld() -> View {
    let signal = create_signal(123);
    // Should print `123`.
    console_log!("{}", signal.get());

    // Update the signal with a new value.
    signal.set(456);

    // Should print `456`.
    console_log!("{}", signal.get());
    // `Signal<T>` also implements `Display` so this is the same as the above.
    console_log!("{signal}");

    view! {
        p { "Hello, world!" }
    }
}

#[component]
fn App() -> View {
    view! {
        Wrapper {
            p { "Nested children" }
        }

        // Components can be at the top-level of a view.
        HelloWorld()
        Hello(name="Sycamore".into())

        // Or they can be nested.
        div {
            HelloWorld()
        }
    }
}

fn main() {
    sycamore::render(App);
}
