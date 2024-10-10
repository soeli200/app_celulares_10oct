use yew::prelude::*;
use yew::Callback;
use web_sys::{HtmlInputElement, window};
use stylist::Style;

#[function_component(PaginaRegistroUsuario)]
pub fn pagina_registro_usuario() -> Html {
    let stylesheet = Style::new(include_str!("module_css_pagina_registro.css")).unwrap();

    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());
    let valid_form = use_state(|| false);

    let oninput_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let oninput_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let oninput_confirm_password = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            confirm_password.set(input.value());
        })
    };

    use_effect_with_deps(
        {
            let username = username.clone();
            let email = email.clone();
            let password = password.clone();
            let confirm_password = confirm_password.clone();
            let valid_form = valid_form.clone();
            move |_| {
                let is_valid = !username.is_empty()
                    && !email.is_empty()
                    && !password.is_empty()
                    && *password == *confirm_password;
                valid_form.set(is_valid);
                || ()
            }
        },
        (username.clone(), email.clone(), password.clone(), confirm_password.clone()),
    );

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(storage) = window().unwrap().local_storage().unwrap() {
                storage.set_item("username", &username).unwrap();
                storage.set_item("email", &email).unwrap();
                storage.set_item("password", &password).unwrap();
                web_sys::console::log_1(&"User registered successfully!".into());
            }
        })
    };

    html! {
        <div class={stylesheet.get_class_name().to_string()}>
            <form onsubmit={onsubmit}>
                <div>
                    <label for="username">{ "Username" }</label>
                    <input type="text" id="username" value={(*username).clone()} oninput={oninput_username} />
                </div>
                <div>
                    <label for="email">{ "Email" }</label>
                    <input type="email" id="email" value={(*email).clone()} oninput={oninput_email} />
                </div>
                <div>
                    <label for="password">{ "Password" }</label>
                    <input type="password" id="password" value={(*password).clone()} oninput={oninput_password} />
                </div>
                <div>
                    <label for="confirm_password">{ "Confirm Password" }</label>
                    <input type="password" id="confirm_password" value={(*confirm_password).clone()} oninput={oninput_confirm_password} />
                </div>
                <button type="submit" disabled={!*valid_form}>{ "Register" }</button>
            </form>
        </div>
    }
}