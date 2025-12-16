use dioxus::prelude::*;

use components::{
    add_buchung::AddBuchung, app_layout::AppLayout, home::Home,
    jahresuebersicht::Jahresuebersicht, liste_buchung::BuchungListe,
    login::LoginPage, register::RegisterPage, add_abo::AddAbo, list_abo::AboListe,
};

use crate::backend::server_functions::benutzer_fns::existiert_benutzer;

mod backend;
mod components;
mod icons;

static CSS: Asset = asset!("/assets/main.css");

// ... Route Enum bleibt gleich ...
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
        #[route("/jahresuebersicht")]
        Jahresuebersicht{},
        #[route("/abo")]
        AboListe{},
        #[route("/abo/add")]
        AddAbo{}
}

fn main() {
    // NEU: Umgebungsvariablen laden (nur auf dem Server)
    #[cfg(feature = "server")]
    {
        dotenv::dotenv().ok();
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // ... Rest bleibt gleich ...
    use_context_provider(|| Signal::new(false)); 
    
    let check_users = use_resource(move || async move {
        existiert_benutzer().await
    });

    rsx! {
        document::Stylesheet { href: CSS }
        
        match &*check_users.read_unchecked() {
            Some(Ok(_)) => {
                rsx! { Router::<Route> {} }
            },
            Some(Err(e)) => rsx! { div { "Server Fehler: {e}" } },
            None => rsx! { div { "Lade System..." } } 
        }
    }
}