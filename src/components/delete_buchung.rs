use dioxus::prelude::*;
use crate::icons::{Icon, mdi_light};
use crate::backend::server_functions::buchung_fns::delete_buchung;
use crate::backend::models::buchung::Buchung; // Ggf. prüfen, ob das Import nötig ist, hier aber unkritisch

#[component]
pub fn Delete(mut buchung_resource: Resource<Result<Vec<Buchung>, ServerFnError>>, id: String) -> Element {
    rsx!{
        button {
            onclick: move |_| {
                // Wir klonen die ID hier, damit 'id' in der Closure erhalten bleibt
                // und nur der Klon in den async Block wandert.
                let id = id.clone();
                async move {
                    match delete_buchung(id).await {
                        Ok(_) => {
                            buchung_resource.restart();
                        }
                        Err(e) => {
                            // Optional: Fehler loggen
                            println!("Fehler beim Löschen: {:?}", e);
                        }
                    }
                }
            },
            Icon { data: mdi_light::Delete }
        }
    }
}