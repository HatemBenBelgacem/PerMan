use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
    // State für die Eingabefelder
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());

    let on_submit = move |_| {
        if username.read().is_empty() || password.read().is_empty() {
            error_msg.set("Bitte fülle alle Felder aus!".to_string());
        } else {
            // Hier würde deine Login-Logik (API-Call) stehen
            println!("Loginversuch: {} mit Passwort: {}", username, password);
            error_msg.set("".to_string());
        }
    };

    rsx! {
        div { class: "login-container",
            h2 { "Login" }
            
            if !error_msg.read().is_empty() {
                p { color: "red", "{error_msg}" }
            }

            form { 
                prevent_default: "onsubmit", // Verhindert Neuladen der Seite
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
                button { r#type: "submit", "Anmelden" }
            }
        }
    }
}