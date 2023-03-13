use leptos::*;

#[component]
pub fn SimpleCounter(
    cx: Scope,
    initial_value: i32,
    step: i32
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    view! {cx,
        <div>
            <button on:click= move |_| set_value(0)>"Clear"</button>
            <button on:click= move |_| set_value.update(|value| *value -= step)>"-"{step}</button>
            <span>"Value: " {value} "!"</span>
            <button on:click = move |_| set_value.update(|value| *value += step)>"+"{step}<button>
        </div>
    }
}