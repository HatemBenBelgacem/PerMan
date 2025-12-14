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

    use_context_provider(|| Signal::new(false));

    let nav = use_navigator();

    let check_users = use_resource(move || async move {
        if let Ok(exists) = existiert_benutzer().await {
            if !exists{
                nav.replace(Route::RegisterPage {});
            }
        }
    });

    rsx! {
        document::Stylesheet { href: CSS}
        Router::<Route> {}
    }
}
