use dioxus::prelude::*;
use crate::backend::server_functions::buchung_fns::{update_buchung}; 
use crate::backend::models::buchung::{BuchungsIntervall, Art};
use chrono::NaiveDate;

#[component] 
pub fn EditBuchung(id: String) -> Element {
    let nav = use_navigator();

    // 1. Ressource zum Laden der bestehenden Daten
    let buchung_resource = use_resource(move || {
        let id = id.clone();
        async move {

        }
    });

    let mut datum = use_signal(|| String::new());
    let mut bezeichnung = use_signal(|| String::new());
    let mut betrag = use_signal(|| String::new());
    let mut intervall = use_signal(|| BuchungsIntervall::Einmalig);
    let mut art = use_signal(|| Art::Ausgaben);

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
                onclick: move |_| async move {
                    let save_datum = datum.read().clone();
                    let save_bezeichnung = bezeichnung.read().clone();
                    let save_betrag = betrag.read().parse::<f64>().unwrap_or(0.0);
                    let save_intervall = intervall.read().clone();
                    let save_art = art.read().clone();

                    if let Ok(parsed_datum) = NaiveDate::parse_from_str(&save_datum, "%Y-%m-%d") {
                        match update_buchung(
                                parsed_datum,
                                save_bezeichnung.clone(),
                                save_betrag.clone(),
                                save_intervall.clone(),
                                save_art.clone(),
                            )
                            .await
                        // WICHTIG: Fehler ausgeben!
                        {
                            Ok(new_uuid) => {
                                let buchung = Buchung {
                                    id: new_uuid,
                                    datum: parsed_datum,
                                    bezeichnung: save_bezeichnung,
                                    betrag: save_betrag,
                                    intervall: Some(save_intervall),
                                    art: Some(save_art),
                                };
                                list_signal.write().push(buchung);
                                nav.push("/buchung");
                            }
                            Err(e) => {
                                println!("FEHLER beim Speichern: {:?}", e);
                            }
                        }
                    }
                    datum.set(String::new());
                    bezeichnung.set(String::new());
                    betrag.set(String::new());
                },
                disabled: if bezeichnung.read().trim().is_empty() { true } else { false },
                "Speichern"
            }
            Link { class: "btn", to: "/buchung", "Abbrechen" }
        }
    }
}