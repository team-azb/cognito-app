use yew::prelude::*;

#[function_component(Content)]
pub(crate) fn content() -> Html {
    html! {
        <div>
            <h1>{"main content for authenticated user"}</h1>
        </div>
    }
}
