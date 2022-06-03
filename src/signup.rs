use yew::prelude::*;

#[function_component(SignUp)]
pub(crate) fn signUp() -> Html {
    html! {
        <div>
            <h1>{ "Sign up on the App!!" }</h1>
            <hr/>
            <div>
                <p>{"emailを入力"}</p>
                <input type="email"/>
            </div>
            <div>
                <p>{"パスワードを入力"}</p>
                <input type="password"/>
            </div>
            <input type="submit" value="サインアップ" />
        </div>
    }
}
