use dioxus::prelude::*;
use crate::backend::server_functions::buchung_fns::{update_buchung};
use crate::backend::models::buchung::{BuchungsIntervall, Art};
use chrono::NaiveDate;



pub fn EditBuchung(id:String) -> Element {
    let nav = use_navigator();

    let buchung_data = use_resource(move|| {
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

            // ... Analog zu AddBuchung die weiteren Felder ...
            button {
                class: "btn",
                onclick: move |_| {
                    let id = id.clone();
                    async move {
                        if let Ok(parsed_date) = NaiveDate::parse_from_str(
                            &datum.read(),
                            "%Y-%m-%d",
                        ) {
                            let res = update_buchung(
                                    id,
                                    parsed_date,
                                    bezeichnung.read().clone(),
                                    betrag.read().parse().unwrap_or(0.0),
                                    intervall.read().clone(),
                                    art.read().clone(),
                                )
                                .await;
                            if res.is_ok() {
                                nav.push("/buchung");
                            }
                        }
                    }
                },
                "Änderungen speichern"
            }
            Link { class: "btn", to: "/buchung", "Abbrechen" }
        }
    }
}