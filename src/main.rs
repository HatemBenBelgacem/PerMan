use dioxus::prelude::*;

use components::{liste_buchung::BuchungListe, add_buchung::AddBuchung, app_layout::AppLayout, home::Home};

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
    AddBuchung{}

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











