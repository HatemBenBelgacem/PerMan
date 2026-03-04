use dioxus::prelude::*;
use crate::components::geruest::{header::Header, nav::Nav};
use crate::Route;
// WICHTIG: Import hinzufügen!
use crate::backend::server_functions::benutzer_fns::existiert_benutzer; 

#[component]
pub fn AppLayout() -> Element {
    let is_logged_in = use_context::<Signal<bool>>();
    let nav = use_navigator();

    // 1. Check: Gibt es überhaupt Benutzer?
    let users_check = use_resource(move || async move {
        existiert_benutzer().await
    });

    // Wir warten kurz auf das Ergebnis des Checks
    match &*users_check.read() {
        Some(Ok(exists)) => {
            if !*exists {
                // Wenn KEIN Benutzer existiert -> Ab zur Registrierung
                // use_effect verhindert Fehler während des Renderns
                use_effect(move || {
                    nav.replace(Route::RegisterPage {});
                });
                return rsx! {
                    div { "Leite zur Einrichtung weiter..." }
                };
            }
        },
        _ => return rsx! {
            div { "Lade..." }
        } // Zeige Lade-Text solange der Check läuft
    }

    // 2. Check: Ist der Benutzer eingeloggt? (Nur wenn Benutzer existieren)
    if !*is_logged_in.read() {
        use_effect(move || {
            nav.replace(Route::LoginPage {});
        });
        return rsx!{
            div { "Bitte anmelden..." }
        };
    }
    
    rsx!{
        div { class: "container",
            div { class: "header", Header {} }

            div { class: "menu", Nav {} }
            div { class: "content", Outlet::<Route> {} }
        }
    }
}