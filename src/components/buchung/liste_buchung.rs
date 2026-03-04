use dioxus::{prelude::*};

use crate::backend::server_functions::{buchung_fns::liste_buchung, buchung_fns::total_buchung_heute};
use crate::backend::{server_functions::buchung_fns::total_buchung};
use crate::backend::server_functions::buchung_fns::total_buchung_einahmen;

use crate::components::buchung::{delete_buchung::Delete};
use num_format::{Buffer, Locale};

#[component]
pub fn BuchungListe() -> Element {
let buchung_resource = use_resource(move || async move {liste_buchung().await});

let total_ausgaben = use_resource(move || async move {total_buchung().await.unwrap_or(0.0)});
let total_einahmen = use_resource(move || async move {total_buchung_einahmen().await.unwrap_or(0.0)});
let total_heute = use_resource(move || async move {total_buchung_heute().await.unwrap_or(0.0)});

let ausgaben = total_ausgaben.read().unwrap_or(0.0);
let einnahmen = total_einahmen.read().unwrap_or(0.0);
let heute = total_heute.read().unwrap_or(0.0);
let differenz = einnahmen - ausgaben;

fn format_betrag(wert: f64) -> String {
    let mut buf = Buffer::default();
    let ganzzahl = wert as i64;

    buf.write_formatted(&ganzzahl, &Locale::de_CH); 

    // Nachkommastellen berechnen
    let dezimal = ((wert.abs() - ganzzahl.abs() as f64) * 100.0).round() as i64;

    format!("{}.{:02}", buf.as_str(), dezimal)
}

fn format_datum(datum: chrono::NaiveDate) -> String {
    datum.format("%d.%m.%Y").to_string()
}
    rsx! {
        div { class: "functions",
            Link { class: "btn", to: "/buchung/add", "Neu buchung" }
            Link { class: "btn", to: "/", "Zurück" }
        }

        div { class: "liste",
            match &*buchung_resource.read_unchecked() {
                // 1. Erfolgreich geladen (Some -> Ok)
                Some(Ok(buchung)) => rsx! {
                    if buchung.is_empty() {
                        div { "Keine Buchung gefunden." }
                    } else {
                        table {

                            // 2. Fehler beim Laden vom Server (Some -> Err)

                            // 3. Daten werden noch geladen (None)
                            // Hier lag der Fehler: Es ist einfach "None", nicht "Some(None)"
                            thead {
                                tr {
                                    th { "Datum" }
                                    th { "Bezeichnung" }
                                    th { "Intervall" }
                                    th { "Art" }
                                    th { class: "betrag", "pro Monat" }
                                    th { class: "betrag", "pro Jahr" }
                                    th { class: "aktion", "Aktion" }
                                }
                            }
                            tbody {
                                for k in buchung {
                                    tr { key: "{k.id}",
                                        td { "{format_datum(k.datum)}" }
                                        td { "{k.bezeichnung}" }
                                        td {
                                            "{k.intervall.as_ref().map(|i| i.to_string()).unwrap_or_else(|| \"-\".to_string())}"
                                        }
                                        td { "{k.art.as_ref().map(|i| i.to_string()).unwrap_or_else(|| \"-\".to_string())}" }
                                        td { class: "betrag", "{format_betrag(k.betrag)}" }
                                        td { class: "betrag", "{format_betrag(k.betrag*12.0)}" }
                                        td { class: "aktion",
                                            Delete { buchung_resource, id: k.id.clone() }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { style: "color: red;", "Fehler beim Laden der Buchung: {e}" }
                },
                None => rsx! {
                    div { "Lade Daten..." }
                },
            }
        }

    }
}