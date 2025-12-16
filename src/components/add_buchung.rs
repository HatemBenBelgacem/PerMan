
use dioxus::prelude::*;
use crate::backend::server_functions::buchung_fns::speichere_buchung;
use crate::backend::models::buchung::{Buchung, BuchungsIntervall, Art};
use chrono::{NaiveDate, Local};


#[component]
pub fn AddBuchung() -> Element {
    let mut datum = use_signal(|| Local::now().format("%Y-%m-%d").to_string());
    let mut bezeichnung = use_signal(|| String::new());
    let mut betrag = use_signal(|| String::new());
    let mut intervall = use_signal(|| BuchungsIntervall::Einmalig);
    let mut art = use_signal(|| Art::Ausgaben);

    let mut list_signal = use_signal(|| Vec::<Buchung>::new()); 
    let nav = use_navigator();

    rsx! {
        div { class:"add_form",
            label { "Fällig per" }
            br {  }
            input {  
                r#type: "date",
                value: datum,
                oninput: move |e| datum.set(e.value()),
            }
            br {  }
            label { "Bezeichnung" }
            br {  }
            input {  
                r#type: "text",
                value: bezeichnung,
                oninput: move |e| bezeichnung.set(e.value()),
            }

            label { "Intervall" }
            br { }
            select {
                class: "input",
                oninput: move |evt| {
                    let neues_intervall = match evt.value().as_str() {
                        "Taeglich" => BuchungsIntervall::Taeglich,
                        "Woechentlich" => BuchungsIntervall::Woechentlich,
                        "Monatlich" => BuchungsIntervall::Monatlich,
                        "Jaehrlich" => BuchungsIntervall::Jaehrlich,
                        _ => BuchungsIntervall::Einmalig,
                    };
                    intervall.set(neues_intervall);
                },
                // Die Optionen
                option { value: "Einmalig", "Einmalig" }
                option { value: "Taeglich", "Täglich" }
                option { value: "Woechentlich", "Wöchentlich" }
                option { value: "Monatlich", "Monatlich" }
                option { value: "Jaehrlich", "Jährlich" }
            }
            br { }

            label { "Art" }
            br {  }
            select {  
                class: "input .material-symbols-outlined",
                oninput: move |evt| {
                    let neue_art = match evt.value().as_str() {
                        "Ausgaben" => Art::Ausgaben,
                        "Einahmen" => Art::Einahmen,
                        _ => Art::Ausgaben,
                    };
                    art.set(neue_art);
                },
                option { value: "Ausgaben", "Ausgaben" }
                option { value: "Einahmen", "Einahmen" }
            }
            br { }
            label { "Betrag" } 
            br {  }
            input { 
                r#type: "number",
                value: betrag,
                oninput: move |e| betrag.set(e.value()),
            } 
            br {  }

            button {  class:"btn",
                onclick: move |_| async move {
                    let save_datum = datum.read().clone();
                    let save_bezeichnung = bezeichnung.read().clone();
                    let save_betrag = betrag.read().parse::<f64>().unwrap_or(0.0);
                    let save_intervall = intervall.read().clone();
                    let save_art = art.read().clone();

                    if let Ok(parsed_datum) = NaiveDate::parse_from_str(&save_datum, "%Y-%m-%d") {
                        match speichere_buchung(parsed_datum, save_bezeichnung.clone(), save_betrag.clone(), save_intervall.clone(), save_art.clone()).await {
                            Ok(id) => {
                                let buchung = Buchung {
                                    id,
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
                                // WICHTIG: Fehler ausgeben!
                                println!("FEHLER beim Speichern: {:?}", e); 
                                // Optional: Alert im Browser anzeigen (via web-sys oder gloo)
                            }
                        }
                    }
                    
                    // HIER: Das Ergebnis loggen
            
                    datum.set(String::new());
                    bezeichnung.set(String::new());
                    betrag.set(String::new());
                },
                disabled: if bezeichnung.read().trim().is_empty() { true } else { false },
                "Speichern"
            }
            // ... (Rest des Codes für die Liste)
            Link { 
                class:"btn",
                to: "/", "Abbrechen" 
            }
        }
        
    }
}


