// src/components/login.rs
use dioxus::prelude::*;
use crate::Route; 
// Importieren Sie die neue Funktion
use crate::backend::server_functions::benutzer_fns::login_check; 

#[component]
pub fn LoginPage() -> Element {
    let mut is_logged_in = use_context::<Signal<bool>>();
    let nav = use_navigator();

    let mut benutzername = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());

    // Machen Sie den Handler asynchron (async move)
    let on_submit = move |_| async move {
        let benutzer = benutzername.read().clone();
        let pass = password.read().clone();

        if benutzer.is_empty() || pass.is_empty() {
            error_msg.set("Bitte fülle alle Felder aus!".to_string());
            return;
        }

        // Server-Call statt hartkodierter Vergleich
        match login_check(benutzer, pass).await {
            Ok(true) => {
                is_logged_in.set(true);
                nav.replace(Route::Home {});
            }
            Ok(false) => {
                error_msg.set("Falsche Zugangsdaten".to_string());
            }
            Err(e) => {
                error_msg.set(format!("Serverfehler: {}", e));
            }
        }
    };

    rsx! {
        div { class: "login-container",
            h2 { "Anmeldung PerMan" }
            
            if !error_msg.read().is_empty() {
                p { style: "color: red;", "{error_msg}" }
            }

            form { 
                // prevent_default und onsubmit handler anpassen
                prevent_default: "onsubmit",
                onsubmit: on_submit,

                div {
                    label { "Benutzername:" }
                    input { 
                        value: "{benutzername}", 
                        oninput: move |evt| benutzername.set(evt.value()) 
                    }
                }
                div {
                    label { "Passwort:" }
                    input { 
                        r#type: "password", 
                        value: "{password}", 
                        oninput: move |evt| password.set(evt.value()) 
                    }
                }
                button { 
                    class: "btn", 
                    style: "width: 100%; margin-top: 10px;",
                    r#type: "submit", 
                    "Anmelden" 
                }
            }
        }
    }
}