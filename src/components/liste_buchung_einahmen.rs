use dioxus::{prelude::*};
use crate::backend::server_functions::buchung_fns::{liste_buchung_einahmen, total_buchung_einahmen};
use crate::components::delete_buchung::Delete;
use num_format::{Buffer, Locale};


#[component]
pub fn BuchungListeEinahmen() -> Element {
let buchung_resource = use_resource(move || async move {liste_buchung_einahmen().await});
let gesamt_summe = use_resource(move || async move { 
    total_buchung_einahmen().await.unwrap_or(0.0) 
});
    
fn format_betrag(wert: f64) -> String {
    let mut buf = Buffer::default();
    let ganzzahl = wert as i64;
    // Locale::de_CH macht 1'000 (Schweiz)
    // Locale::de_DE macht 1.000 (Deutschland)
    buf.write_formatted(&ganzzahl, &Locale::de_CH); 

    // Nachkommastellen berechnen
    let dezimal = ((wert.abs() - ganzzahl.abs() as f64) * 100.0).round() as i64;

    format!("{}.{:02}", buf.as_str(), dezimal)
} 
    
    
    rsx! {
        div { class: "liste",
            h3 { "Einahmen" }
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
                                    th { class:"betrag", "pro Monat" }
                                    th { class:"betrag", "pro Jahr" }
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
                                        td { class:"betrag", "{format_betrag(k.betrag)}" }
                                        td { class:"betrag", "{format_betrag(k.betrag *12.0)}" }
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
            style:"color:green;",
            p { "Total Einahmen: {gesamt_summe.read().unwrap_or(0.0):.2}" }
        }
       
    }
}