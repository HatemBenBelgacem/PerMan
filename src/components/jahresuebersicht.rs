use dioxus::prelude::*;

use crate::backend::server_functions::buchung_fns::liste_buchung;

#[component]
pub fn Jahresuebersicht() -> Element {
    let buchung_resource = use_resource(move || async move { liste_buchung().await });

    let monate = [("01", "Januar"), ("02", "Februar"), ("03", "März"), ("04","April"), ("05", "Mai"), ("06", "Juni"), ("07", "Juli"), ("08", "August"), ("09", "September"), ("10", "Oktober"), ("11","November"), ("12", "Dezember")];
    rsx! {
        match &*buchung_resource.read_unchecked() {
            Some(Ok(buchung)) => rsx! {
                if buchung.is_empty() {
                    div {"Keine Buchung"}
                } else {
                    div { class:"monats_container",
                        for (index_str, m) in monate {
                            div { class:"monats_wrapper",
                               table {
                                    thead {
                                        tr {
                                            th { "{m}" }
                                        }
                                    }
                                    for b in buchung.iter().filter(|b| b.datum.format("%m").to_string() == *index_str) {
                                        
                                            tbody {
                                            tr {
                                                td { "{b.betrag:.2}" }
                                            }
                                        
                                    }
                                    
                                    
                                }
                            }
                        }

                        }
                    }

                }
            },
            Some(Err(e)) => rsx! {
                div{
                    "Fehler beim Laden {e}"
                }
            },
            None => rsx! {
                div{"Lade Daten..."}
            }

        }
    }
}
