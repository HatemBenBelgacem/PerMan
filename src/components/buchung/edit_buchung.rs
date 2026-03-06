use dioxus::prelude::*;
// Stelle sicher, dass `Buchung` hier ebenfalls importiert wird!
use crate::backend::models::buchung::{Buchung, BuchungsIntervall, Art};
// Die Funktion `update_buchung` sollte so angepasst werden, dass sie eine `id` akzeptiert.
use crate::backend::server_functions::buchung_fns::{update_buchung, get_buchung}; 
use chrono::NaiveDate;

#[component] 
pub fn EditBuchung(id: String) -> Element {
    let nav = use_navigator();
    
    // Hole dir das globale oder kontextbezogene Signal für deine Buchungsliste
    // let mut list_signal = use_context::<Signal<Vec<Buchung>>>();

    let mut datum = use_signal(|| String::new());
    let mut bezeichnung = use_signal(|| String::new());
    let mut betrag = use_signal(|| String::new());
    let mut intervall = use_signal(|| BuchungsIntervall::Einmalig);
    let mut art = use_signal(|| Art::Ausgaben);

    // 1. Ressource zum Laden der bestehenden Daten
    let _buchung_resource = use_resource(move || {
        let id_clone = id.clone();
        async move {
            // Pseudocode: Lade die Buchung vom Server
            // if let Ok(existing) = get_buchung(id_clone).await {
            //     datum.set(existing.datum.format("%Y-%m-%d").to_string());
            //     bezeichnung.set(existing.bezeichnung);
            //     betrag.set(existing.betrag.to_string());
            // }
        }
    });

    rsx! {
        div { class: "add_form",
            h2 { "Buchung bearbeiten" }

            label { "Bezeichnung" }
            input {
                value: "{bezeichnung}",
                oninput: move |e| bezeichnung.set(e.value()),
            }

            button {
                class: "btn",
                // Deaktiviere den Button, wenn die Bezeichnung leer ist
                disabled: bezeichnung.read().trim().is_empty(),
                onclick: move |_| async move {
                    let save_datum = datum.read().clone();
                    let save_bezeichnung = bezeichnung.read().clone();
                    let save_betrag = betrag.read().parse::<f64>().unwrap_or(0.0);
                    let save_intervall = intervall.read().clone();
                    let save_art = art.read().clone();

                    if let Ok(parsed_datum) = NaiveDate::parse_from_str(&save_datum, "%Y-%m-%d") {
                        // WICHTIG: Die ID muss beim Update übergeben werden!
                        match update_buchung(
                                id.clone(), // <- HIER FEHLTE DIE ID ZUVOR
                                parsed_datum,
                                save_bezeichnung.clone(),
                                save_betrag,
                                save_intervall.clone(),
                                save_art.clone(),
                            )
                            .await
                        {
                            Ok(_) => {
                                // Hier aktualisierst du das Element in deiner Liste,
                                // anstatt es neu hinzuzufügen (.push), z.B. über `.retain` oder Iteration über `list_signal.write()`.
                                nav.push("/buchung");
                            }
                            Err(e) => {
                                eprintln!("FEHLER beim Speichern der Änderungen: {:?}", e);
                            }
                        }
                    }
                },
                "Speichern"
            }
            Link { class: "btn", to: "/buchung", "Abbrechen" }
        }
    }
}