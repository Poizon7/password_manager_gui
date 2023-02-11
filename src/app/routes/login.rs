use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html!{
        <main>
            <form>
                <fieldset>
                    <legend>{"Login"}</legend>
                    <lable for="pwd">
                        <span>{"Password: "}</span>
                        <strong><span aria-label="required">{"*"}</span></strong>
                    </lable>
                    <input type="password" id="pwd" name="password" />
                </fieldset>
            </form>
        </main>
    }
}
