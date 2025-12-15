use dioxus::prelude::*;
use crate::backend::server_functions::abo_fns::speichere_abo;
use crate::backend::models::abo::Abo;

use chrono::{NaiveDate, Local};

#[component]
pub fn AddAbo() -> Element {
    let mut bezeichnung = use_signal(|| String::new());
    let mut begin = use_signal(|| Local::now().format("%Y-%m-%d").to_string());
    let mut dauer = use_signal(|| String::new());
    let mut knd_frist = use_signal(|| String::new());
    let nav = use_navigator();

    let mut list_signal = use_signal(|| Vec::<Abo>::new());
    rsx!{
        div {
            class:"add_form",
            label{ "Bezeichnung" }
            br{}
            input{
                r#type:"text",
                value: bezeichnung,
                oninput: move |e| bezeichnung.set(e.value()),
            }
            br{}
            label{"Begin"}
            br{}
            input{
                r#type:"date",
                value:begin,
                oninput: move |e| begin.set(e.value()),
            }
            br{}
            label{"Dauer"}
            br{}
            input{
                r#type:"number",
                value:dauer,
                oninput: move |e| dauer.set(e.value()),
            }
            br{}
            label{"Kündigungsfrist"}
            br{}
            input{
                r#type:"number",
                value:knd_frist,
                oninput: move |e| knd_frist.set(e.value()),
            }
            br{}

            button {
                class:"btn",
                onclick: move |_| async move {
                    let save_bezeichnung = bezeichnung.read().clone();
                    let save_begin = begin.read().clone();
                    let save_dauer = dauer.read().clone();
                    let save_knd_frist = knd_frist.read().clone();

                    if let Ok(parsed_datum) = NaiveDate::parse_from_str(&save_begin, "%Y-%m-%d") {
                        match speichere_abo(save_bezeichnung.clone(), parsed_datum, save_dauer.clone(), save_knd_frist.clone()).await {
                            Ok(new_uuid) => {
                                let abo = Abo {
                                    id: new_uuid,
                                    bezeichnung: save_bezeichnung,
                                    begin: parsed_datum,
                                    dauer: save_dauer,
                                    knd_frist: save_knd_frist,
                                };
                                list_signal.write().push(abo);
                                nav.push("/")
                            }
                            Err(e) => {
                                println!("Fehler: {:?}", e);
                            }
                        }
                    }    
                },
                disabled: if bezeichnung.read().trim().is_empty() { true } else {false},
                "Speichern"
            }
            Link{
                class:"btn",
                to:"/", "Abbrechen"
            }
        }
    }
}