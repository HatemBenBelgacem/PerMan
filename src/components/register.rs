use dioxus::prelude::*;
use crate::backend::server_functions::benutzer_fns::speichere_benutzer;
use crate::Route;

#[component]
pub fn RegisterPage() -> Element {
    let nav = use_navigator();
    let mut username = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());

    let on_submit = move |_| async move {
        let user = username.read().clone();
        let mail = email.read().clone();
        let pass = password.read().clone();

        if user.is_empty() || pass.is_empty() {
            error_msg.set("Bitte alle Felder ausfüllen".to_string());
            return;
        }

        // Benutzer speichern
        match speichere_benutzer(user, mail, pass).await {
            Ok(_) => {
                // Nach erfolgreicher Registrierung zum Login leiten
                nav.replace(Route::LoginPage {});
            }
            Err(e) => {
                error_msg.set(format!("Fehler beim Erstellen: {}", e));
            }
        }
    };

    rsx! {
        div { class: "login-container", // Wir nutzen denselben CSS-Style wie beim Login
            h2 { "Ersteinrichtung: Admin erstellen" }
            
            if !error_msg.read().is_empty() {
                p { style: "color: red;", "{error_msg}" }
            }

            form { 
                prevent_default: "onsubmit", 
                onsubmit: on_submit,

                div {
                    label { "Benutzername:" }
                    input { value: "{username}", oninput: move |e| username.set(e.value()) }
                }
                div {
                    label { "Email:" }
                    input { value: "{email}", oninput: move |e| email.set(e.value()) }
                }
                div {
                    label { "Passwort:" }
                    input { r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()) }
                }
                button { 
                    class: "btn",
                    style: "width: 100%; margin-top: 10px;",
                    r#type: "submit", 
                    "Registrieren" 
                }
            }
        }
    }
}