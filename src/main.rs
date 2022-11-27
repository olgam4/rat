use leptos::*;

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> Element {
    let (value, set_value) = create_signal(cx, initial_value);

    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        cx,
        <div class="flex absolute top-0 bottom-0 right-0 left-0 justify-center items-center space-x-2">
            <button class="px-4 py-1 rounded bg-blue-200" on:click=clear>"Clear"</button>
            <button class="px-4 py-1 rounded bg-red-200" on:click=decrement>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button class="px-4 py-1 rounded bg-green-200" on:click=increment>"+1"</button>
        </div>
    }
}

pub fn main() {
    mount_to_body(|cx| view! { cx, <SimpleCounter initial_value=3 /> })
}
