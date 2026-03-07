use dioxus::prelude::*;
use chrono::NaiveDate;

use crate::backend::models::buchung::{BuchungsIntervall, Art};
use crate::backend::server_functions::buchung_fns::{update_buchung, get_buchung}; 

#[component] 
pub fn EditBuchung(id: String) -> Element {
    let nav = use_navigator();

    let mut datum = use_signal(|| String::new());
    let mut bezeichnung = use_signal(|| String::new());
    let mut betrag = use_signal(|| String::new());
    let mut intervall = use_signal(|| BuchungsIntervall::Einmalig);
    let mut art = use_signal(|| Art::Ausgaben);

    // 1. KORREKTUR: Wir klonen die ID BEVOR wir sie in die erste Closure (use_resource) bewegen
    let id_for_fetch = id.clone();

    let buchung_resource = use_resource(move || {
        // Wir nehmen den Klon und klonen ihn für den inneren async-Block
        let fetch_id = id_for_fetch.clone();
        async move {
            if let Ok(existing_buchung) = get_buchung(fetch_id).await {
                datum.set(existing_buchung.datum.format("%Y-%m-%d").to_string());
                bezeichnung.set(existing_buchung.bezeichnung);
                betrag.set(existing_buchung.betrag.to_string());
                
                if let Some(i) = existing_buchung.intervall {
                    intervall.set(i);
                }
                if let Some(a) = existing_buchung.art {
                    art.set(a);
                }
            }
        }
    });

    rsx! {
        div { class: "add_form",
            h2 { "Buchung bearbeiten" }

            if buchung_resource.cloned().is_none() {
                p { "Lade Daten..." }
            } else {
                label { "Datum" }
                input {
                    r#type: "date",
                    value: "{datum}",
                    oninput: move |e| datum.set(e.value()),
                }

                label { "Bezeichnung" }
                input {
                    r#type: "text",
                    value: "{bezeichnung}",
                    oninput: move |e| bezeichnung.set(e.value()),
                }

                label { "Betrag" }
                input {
                    r#type: "number",
                    step: "0.01",
                    value: "{betrag}",
                    oninput: move |e| betrag.set(e.value()),
                }

                button {
                    class: "btn",
                    disabled: bezeichnung.read().trim().is_empty() || datum.read().trim().is_empty(),
                    onclick: move |_| {
                        // 2. KORREKTUR: Da die erste Closure nur den Klon verbraucht hat,
                        // gehört das Original `id` noch uns und kann hier verwendet werden.
                        let save_id = id.clone();
                        async move {
                            let save_datum = datum.read().clone();
                            let save_bezeichnung = bezeichnung.read().clone();
                            let save_betrag = betrag.read().parse::<f64>().unwrap_or(0.0);
                            let save_intervall = intervall.read().clone();
                            let save_art = art.read().clone();

                            if let Ok(parsed_datum) = NaiveDate::parse_from_str(
                                &save_datum, // Hier übergeben wir unsere sichere Kopie
                                "%Y-%m-%d",
                            ) {
                                match update_buchung(
                                        save_id,
                                        parsed_datum,
                                        save_bezeichnung,
                                        save_betrag,
                                        save_intervall,
                                        save_art,
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        nav.push("/buchung");
                                    }
                                    Err(e) => {
                                        eprintln!("FEHLER beim Speichern: {:?}", e);
                                    }
                                }
                            } else {
                                eprintln!("Ungültiges Datumsformat");
                            }
                        }
                    },
                    "Speichern"
                }
                Link { class: "btn", to: "/buchung", "Abbrechen" }
            }
        }
    }
}