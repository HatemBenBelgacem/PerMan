use dioxus::{prelude::*};

use crate::backend::server_functions::{buchung_fns::liste_buchung};
use crate::backend::{server_functions::buchung_fns::total_buchung};

use crate::components::delete_buchung::Delete;


#[component]
pub fn BuchungListe() -> Element {
let buchung_resource = use_resource(move || async move {liste_buchung().await});
let gesamt_summe = use_resource(move || async move { 
    total_buchung().await.unwrap_or(0.0) 
});
    rsx! {
        div { 
            class:"functions",
            Link { 
                class:"btn",
                to: "/buchung/add", "Neu buchung",
            }
            Link { 
                class: "btn",
                to: "/", "Zurück" 
            } 
        }
        div { class: "liste",
            match &*buchung_resource.read_unchecked() {
                // 1. Erfolgreich geladen (Some -> Ok)
                Some(Ok(buchung)) => rsx! {
                    if buchung.is_empty() {
                        div { "Keine Buchung gefunden." }
                    } else {
                        table {

                            thead {
                                tr {
                                    th { "Datum" }
                                    th { "Bezeichnung" }
                                    th { "Intervall" }
                                    th { class:"betrag", colspan:"2", "Betrag" }
                                    th { class:"aktion", "Aktion" }
                                }
                            }
                            tbody {
                                for k in buchung {
                                    tr { key: "{k.id}",
                                        td { "{k.datum}" }
                                        td { "{k.bezeichnung}" }
                                        td { 
                                            "{k.intervall.as_ref().map(|i| i.to_string()).unwrap_or_else(|| \"-\".to_string())}" 
                                        }
                                        td { "CHF" }
                                        td { class:"betrag", "{k.betrag:.2}" }
                                        td { class:"aktion", Delete{buchung_resource: buchung_resource, id: k.id} }
                                    }
                                }
                            }
                        }
                    }
                },
                
                // 2. Fehler beim Laden vom Server (Some -> Err)
                Some(Err(e)) => rsx! {
                    div { 
                        style: "color: red;",
                        "Fehler beim Laden der Buchung: {e}" 
                    }
                },

                // 3. Daten werden noch geladen (None)
                // Hier lag der Fehler: Es ist einfach "None", nicht "Some(None)"
                None => rsx! {
                    div { "Lade Daten..." }
                }
            }
        }
        div {  class:"total_summe",
            p { "Total: {gesamt_summe.read().unwrap_or(0.0):.2}" }
        }
       
    }
}