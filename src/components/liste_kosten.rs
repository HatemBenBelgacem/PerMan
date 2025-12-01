use dioxus::{prelude::*};

use crate::backend::{server_functions::kosten_fns::liste_kosten};
use crate::backend::{server_functions::kosten_fns::total_kosten};




#[component]
pub fn KostenListe() -> Element {
    let kosten_resource = use_resource(move || async move {
        liste_kosten().await
    });
let gesamt_summe = use_resource(move || async move { 
    total_kosten().await.unwrap_or(0.0) 
});
    rsx! {
        div { class:"btn",
            class:"functions",
            Link { 
                class:"btn",
                to: "/kosten/add", "Neu kosten",
            }
            Link { 
                class: "btn",
                to: "/", "Zurück" 
            } 
        }
        div { class: "liste_kosten",
            match &*kosten_resource.read_unchecked() {
                // 1. Erfolgreich geladen (Some -> Ok)
                Some(Ok(kosten)) => rsx! {
                    if kosten.is_empty() {
                        div { "Keine Kosten gefunden." }
                    } else {
                        table {

                            thead {
                                tr {
                                    th { "Datum" }
                                    th { "Bezeichnung" }
                                    th { "Betrag" }
                                }
                            }
                            tbody {
                                for k in kosten {
                                    tr { key: "{k.id}",
                                        td { "{k.datum}" }
                                        td { "{k.bezeichnung}" }
                                        td { "{k.betrag}" }
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
                        "Fehler beim Laden der Kosten: {e}" 
                    }
                },

                // 3. Daten werden noch geladen (None)
                // Hier lag der Fehler: Es ist einfach "None", nicht "Some(None)"
                None => rsx! {
                    div { "Lade Daten..." }
                }
            }
        }
        div {  
            p { "Total: {gesamt_summe.read().unwrap_or(0.0):.2} CHF" }
        }
       
    }
}