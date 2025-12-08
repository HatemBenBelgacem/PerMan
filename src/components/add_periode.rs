
use dioxus::prelude::*;
use crate::backend::server_functions::periode_fns::speichere_periode;
use crate::backend::models::periode::Periode;




#[component]
pub fn AddPeriode() -> Element {
let mut bezeichnung = use_signal(|| String::new());
let mut list_signal = use_signal(|| Vec::<Periode>::new());
let nav = use_navigator();
    rsx!{
        div { class: "add_form",
            form {
                label {"Bezeichnung"}
                br{}
                input {
                    r#type:"text",
                    value: bezeichnung,
                    oninput: move |e| bezeichnung.set(e.value()),
                }
            }

            button {
                class: "btn",

                disabled: bezeichnung.read().trim().is_empty(),
                onclick: move |_| async move {
                    let bez_val = bezeichnung.read().clone();
                

                    match speichere_periode(bez_val.clone()).await {
                        Ok(id) => {
                            list_signal.write().push(
                                Periode {
                                    id,
                                    bezeichnung: bez_val,
                                }
                            );
                            nav.push("/");

                            bezeichnung.set(String::new());

                        }
                        Err(e) => {
                            print!("Fehler beim Speichern {:?}", e);
                        }
                    }
                },
                "Speichern"
            }

        }
    }

}