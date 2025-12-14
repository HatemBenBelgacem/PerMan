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



#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(false)); // is_logged_in status
    
    // Server Health Check beim Start
    let check_users = use_resource(move || async move {
        existiert_benutzer().await
    });

    rsx! {
        document::Stylesheet { href: CSS }
        
        match &*check_users.read_unchecked() {
            Some(Ok(_)) => {
                // Server läuft -> Router starten (ohne spezielle Config)
                // Die Logik für den Redirect passiert jetzt im AppLayout
                rsx! { Router::<Route> {} }
            },
            Some(Err(e)) => rsx! { div { "Server Fehler: {e}" } },
            None => rsx! { div { "Lade System..." } } 
        }
    }
}
