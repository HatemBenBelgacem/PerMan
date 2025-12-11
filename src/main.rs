use dioxus::prelude::*;

use components::{liste_buchung::BuchungListe, add_buchung::AddBuchung, app_layout::AppLayout, home::Home, add_periode::AddPeriode, liste_periode::PeriodenListe, jahresuebersicht::Jahresuebersicht};

mod backend;
mod components;
mod icons;

static CSS: Asset = asset!("/assets/main.css"); 

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
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
    rsx! {
        document::Stylesheet { href: CSS}
        Router::<Route> {}
    }
}











