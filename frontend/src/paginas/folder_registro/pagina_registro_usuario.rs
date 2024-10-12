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
    let username_error = use_state(|| None);
    let email_error = use_state(|| None);
    let password_error = use_state(|| None);
    let confirm_password_error = use_state(|| None);

    let oninput_username = {
        let username = username.clone();
        let username_error = username_error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value.clone());
            if value.is_empty() {
                username_error.set(Some("El nombre de usuario no puede estar vacío".to_string()));
            } else {
                username_error.set(None);
            }
        })
    };

    let oninput_email = {
        let email = email.clone();
        let email_error = email_error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            email.set(value.clone());
            if value.is_empty() {
                email_error.set(Some("El correo electrónico no puede estar vacío".to_string()));
            } else if !value.contains('@') {
                email_error.set(Some("El correo electrónico no es válido".to_string()));
            } else {
                email_error.set(None);
            }
        })
    };

    let oninput_password = {
        let password = password.clone();
        let password_error = password_error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            password.set(value.clone());
            if value.is_empty() {
                password_error.set(Some("La contraseña no puede estar vacía".to_string()));
            } else if value.len() < 6 {
                password_error.set(Some("La contraseña debe tener al menos 6 caracteres".to_string()));
            } else {
                password_error.set(None);
            }
        })
    };

    let oninput_confirm_password = {
        let confirm_password = confirm_password.clone();
        let password = password.clone();
        let confirm_password_error = confirm_password_error.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            confirm_password.set(value.clone());
            if value != *password {
                confirm_password_error.set(Some("Las contraseñas no coinciden".to_string()));
            } else {
                confirm_password_error.set(None);
            }
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
                    && email.contains('@')
                    && !password.is_empty()
                    && password.len() >= 6
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
                web_sys::console::log_1(&"Usuario registrado exitosamente!".into());
            }
        })
    };

    html! {
        <div class={stylesheet.get_class_name().to_string()}>
            <div class="registro-container">
                <form onsubmit={onsubmit}>
                    <div>
                        <label for="username">{ "Nombre de usuario" }</label>
                        <input type="text" id="username" value={(*username).clone()} oninput={oninput_username} />
                        if let Some(error) = &*username_error {
                            <p class="error-message">{ error }</p>
                        }
                    </div>
                    <div>
                        <label for="email">{ "Correo electrónico" }</label>
                        <input type="email" id="email" value={(*email).clone()} oninput={oninput_email} />
                        if let Some(error) = &*email_error {
                            <p class="error-message">{ error }</p>
                        }
                    </div>
                    <div>
                        <label for="password">{ "Contraseña" }</label>
                        <input type="password" id="password" value={(*password).clone()} oninput={oninput_password} />
                        if let Some(error) = &*password_error {
                            <p class="error-message">{ error }</p>
                        }
                    </div>
                    <div>
                        <label for="confirm_password">{ "Confirmar contraseña" }</label>
                        <input type="password" id="confirm_password" value={(*confirm_password).clone()} oninput={oninput_confirm_password} />
                        if let Some(error) = &*confirm_password_error {
                            <p class="error-message">{ error }</p>
                        }
                    </div>
                    <button type="submit" disabled={!*valid_form}>{ "Registrarse" }</button>
                </form>
            </div>
        </div>
    }
}