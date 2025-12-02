use dioxus::prelude::*;

use components::{liste_kosten::KostenListe, add_kosten::AddKosten, app_layout::AppLayout, home::Home};

mod backend;
mod components;

static CSS: Asset = asset!("/assets/main.css"); 

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home{},
    #[route("/kosten")]
    KostenListe {},
    #[route("/kosten/add")]
    AddKosten{}

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











