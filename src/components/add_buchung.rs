
use dioxus::prelude::*;
use crate::backend::server_functions::buchung_fns::speichere_buchung;
use crate::backend::models::buchung::Buchung;
use chrono::{NaiveDate, Local};


#[component]
pub fn AddBuchung() -> Element {
    let mut datum = use_signal(|| Local::now().format("%Y-%m-%d").to_string());
    let mut bezeichnung = use_signal(|| String::new());
    let mut betrag = use_signal(|| String::new());
    let mut sel_periode_id = use_signal(|| 0i64);

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
            br {  }
            label { "Betrag" } 
            br {  }
            input { 
                r#type: "number",
                value: betrag,
                oninput: move |e| betrag.set(e.value()),
            } 
              br {  }

            br {  }

            button {  class:"btn",
                onclick: move |_| async move {
                    let save_datum = datum.read().clone();
                    let save_bezeichnung = bezeichnung.read().clone();
                    let save_betrag = betrag.read().parse::<f64>().unwrap_or(0.0);
                    let save_periode = sel_periode_id.read().clone();

                    if let Ok(parsed_datum) = NaiveDate::parse_from_str(&save_datum, "%Y-%m-%d") {
                                match speichere_buchung(parsed_datum, save_bezeichnung.clone(), save_betrag.clone()).await {
                                    Ok(id) => {
                                        let buchung = Buchung {
                                            id,
                                            datum: parsed_datum,
                                            bezeichnung: save_bezeichnung,
                                            betrag: save_betrag,
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


