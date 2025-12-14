use dioxus::prelude::*;

use components::{
    add_buchung::AddBuchung, add_periode::AddPeriode, app_layout::AppLayout, home::Home,
    jahresuebersicht::Jahresuebersicht, liste_buchung::BuchungListe, liste_periode::PeriodenListe,
    login::LoginPage, register::RegisterPage, 
};

use crate::backend::server_functions::benutzer_fns::existiert_benutzer;

mod backend;
mod components;
mod icons;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/register")]
    RegisterPage{},
    #[route("/login")]
    LoginPage{},

    #[layout(AppLayout)]
        #[route("/")]
        Home{},
        #[route("/buchung")]
        BuchungListe {},
        #[route("/buchung/add")]
        AddBuchung{},
        #[route("/periode")]
        PeriodenListe{},
        #[route("/periode/add")]
        AddPeriode{},
        #[route("/jahresuebersicht")]
        Jahresuebersicht{}
    
}

fn main() {
    dioxus::launch(App);
}

// in src/main.rs

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(false)); // is_logged_in status
    
    // Resource für den Check beim Start
    let check_users = use_resource(move || async move {
        existiert_benutzer().await
    });

    // CSS laden
    rsx! {
        document::Stylesheet { href: CSS }
        
        // Wir warten auf das Ergebnis des Checks
        match &*check_users.read_unchecked() {
            Some(Ok(exists)) => {
                // Wenn Check erfolgreich:
                if !*exists {
                    // Wenn kein Benutzer existiert -> Direkt Register anzeigen (ohne Router-Logik)
                    rsx! { RegisterPage {} }
                } else {
                    // Wenn Benutzer existieren -> Router starten (der dann zum Login leitet via AppLayout)
                    rsx! { Router::<Route> {} }
                }
            },
            Some(Err(e)) => rsx! { div { "Server Fehler: {e}" } }, // Fehler anzeigen!
            None => rsx! { div { "Lade System..." } } // Ladebildschirm
        }
    }
}
