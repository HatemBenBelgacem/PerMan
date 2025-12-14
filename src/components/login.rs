use dioxus::prelude::*;
use crate::Route; // Für die Navigation

#[component]
pub fn LoginPage() -> Element {
    // Zugriff auf den globalen Status, um ihn zu ändern
    let mut is_logged_in = use_context::<Signal<bool>>();
    let nav = use_navigator();

    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());

    let on_submit = move |_| {
        let user = username.read();
        let pass = password.read();

        // Einfache Prüfung (später durch Server-Call ersetzen)
        if user.is_empty() || pass.is_empty() {
            error_msg.set("Bitte fülle alle Felder aus!".to_string());
        } else if *user == "admin" && *pass == "1234" { // Beispiel-Logik
            // 1. Status auf "Eingeloggt" setzen
            is_logged_in.set(true);
            // 2. Zur Startseite leiten
            nav.replace(Route::Home {});
        } else {
            error_msg.set("Falsche Zugangsdaten".to_string());
        }
    };

    rsx! {
        div { class: "login-container",
            h2 { "Anmeldung PerMan" }
            
            if !error_msg.read().is_empty() {
                p { style: "color: red;", "{error_msg}" }
            }

            form { 
                prevent_default: "onsubmit", 
                onsubmit: on_submit,

                div {
                    label { "Benutzername:" }
                    input { 
                        value: "{username}", 
                        oninput: move |evt| username.set(evt.value()) 
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
                    class: "btn", // Deine CSS Klasse nutzen
                    style: "width: 100%; margin-top: 10px;",
                    r#type: "submit", 
                    "Anmelden" 
                }
            }
        }
    }
}