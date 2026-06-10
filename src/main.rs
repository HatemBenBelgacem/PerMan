use dioxus::prelude::*;

use components::buchung::{
    add_buchung::AddBuchung, delete_buchung::Delete, liste_buchung::BuchungListe, 
    jahresuebersicht::Jahresuebersicht,edit_buchung::EditBuchung 
};
use components::geruest::{app_layout::AppLayout, header::Header, home::Home, nav::Nav};
use components::{login::LoginPage, register::RegisterPage};
use components::abo::{add_abo::AddAbo, list_abo::AboListe};
use crate::backend::server_functions::benutzer_fns::existiert_benutzer;
use crate::components::abo::{add_abo, list_abo};

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
        #[route("/buchung/edit/:id")]
        EditBuchung{id:String},
        #[route("/jahresuebersicht")]
        Jahresuebersicht{},
        #[route("/abo")]
        AboListe{},
        #[route("/abo/add")]
        AddAbo{}
}

fn main() {

    println!("🚀 PER-MAN SERVER STARTET AUF PORT 8080...");
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
    use_context_provider(|| Signal::new(true)); 

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}

    }
}