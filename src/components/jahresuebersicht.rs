use dioxus::prelude::*;

use crate::backend::server_functions::buchung_fns::liste_buchung;

#[component]
pub fn Jahresuebersicht() -> Element {
    let buchung_resource = use_resource(move || async move { liste_buchung().await });

    // Ihre Monatsnamen
    let monate = [("01", "Januar"), ("02", "Februar"), ("03", "März"), ("04", "April"), ("05", "Mai"), ("06", "Juni"), ("07", "Juli"), ("08", "August"), ("09", "September"), ("10", "Oktober"), ("11", "November"), ("12", "Dezember")];

    // Hier beginnt der Haupt-Render-Block
    rsx! {
        match &*buchung_resource.read_unchecked() {
            Some(Ok(buchung)) => {
                // Keine Buchungen vorhanden
                if buchung.is_empty() {
                    return rsx! { div {"Keine Buchungen gefunden."} };
                }

                // 1. Sammeln der Buchungen pro Monat
                let mut buchungen_pro_monat: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
                for b in buchung.iter() {
                    let monat_key = b.datum.format("%m").to_string();
                    buchungen_pro_monat.entry(monat_key.clone())
                        .or_insert_with(Vec::new)
                        .push(b.betrag);
                }

                // 2. Maximale Anzahl von Zeilen bestimmen (für die Höhe der Tabelle)
                let max_rows = buchungen_pro_monat.values().map(|v| v.len()).max().unwrap_or(1);
                
                // 3. Generierung der Tabelle
                rsx! {
                    table { 
                        class: "jahresuebersicht_tabelle",
                        style: "width: 100%; border-collapse: collapse;", // Grundlegende CSS für das Layout

                        // Kopfzeile mit allen 12 Monatsnamen
                        thead {
                            tr {
                                for (_, m) in monate.iter() {
                                    th { 
                                        style: "border: 1px solid black; padding: 8px; text-align: center;",
                                        "{m}" 
                                    }
                                }
                            }
                        }

                        // Körper der Tabelle (die Buchungszeilen)
                        tbody {
                            // Erzeuge Zeilen von 0 bis max_rows - 1
                            for row_index in 0..max_rows {
                                tr {
                                    // Gehe durch alle 12 Monate (Spalten)
                                    for (index_str, _) in monate.iter() {
                                        let betraege_im_monat = buchungen_pro_monat.get(&index_str);
                                        
                                        // Prüfe, ob in dieser Zeile ein Betrag für diesen Monat existiert
                                        let zellen_inhalt = if let Some(betraege) = betraege_im_monat {
                                            if row_index < betraege.len() {
                                                // Zeige den Betrag an der korrekten Position
                                                format!("{:.2}", betraege[row_index])
                                            } else {
                                                // Leere Zelle (keine Buchung mehr in dieser Spalte)
                                                String::from("&nbsp;") // &nbsp; um die Zelle nicht komplett zusammenfallen zu lassen
                                            }
                                        } else {
                                            // Leere Zelle (der Monat hat keine Buchungen)
                                            String::from("&nbsp;")
                                        };

                                        td { 
                                            style: "border: 1px solid black; padding: 8px; min-width: 50px;",
                                            dangerous_inner_html: "{zellen_inhalt}" // Wir nutzen dangerous_inner_html für &nbsp;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Some(Err(e)) => rsx! {
                div{"Fehler beim Laden {e}"}
            },
            None => rsx! {
                div{"Lade Daten..."}
            }
        }
    }
}