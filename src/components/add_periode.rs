
use dioxus::prelude::*;
use crate::backend::server_functions::periode_fns::speichere_periode;
use crate::backend::models::periode::Periode;



#[component]
pub fn AddPeriode() -> Element {
let mut list_signal = use_signal(|| Vec::<Periode>);
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
                

                    match speichere_periode(bez_val).await {
                        Ok(id) => {
                            list_signal.write().push(
                                Periode {
                                    id,
                                    bezeichnung: bez_val,
                                }
                            );
                            nav.push("/periode");

                            bezeichnung.set(String::new());

                        }
                        Err(e) => {
                            print!("Fehler beim Speichern {:?}," e);
                        }
                    }
                },
                "Speichern"
            }

        }
    }

}