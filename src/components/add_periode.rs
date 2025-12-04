
use dioxus::prelude::*;
use crate::backend::server_functions::periode_fns::speichere_buchung;



#[component]
pub fn AddPeriode() -> Element {
let mut list_signal = use_signal(|| Vec::<Per>);
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

        }
    }

}