use dioxus::prelude::*;

use crate::backend::server_functions::periode_fns::liste_periode;
use crate::components::delete_periode::Delete;



#[component]
pub fn PeriodenListe() -> Element {
    let periode_resource = use_resource(move || async move {liste_periode().await});

    rsx!{
        div { class: "functions",
            Link {
                class: "btn",
                to: "/periode/add", "Neu Periode"
            }
            Link {
                class: "btn",
                to: "/", "Zurück"
            }
        }

        div{ class:"liste",
            match &*periode_resource.read_unchecked() {
                Some(Ok(periode)) => rsx! {
                    if periode.is_empty() {
                        div{ "Keine Periode gefunden" }
                    } else {
                        table{
                            thead{
                                tr{
                                    th {"Bezeichnung"}
                                    th {class: "aktion", "Aktion"}
                                }
                            }
                            tbody{
                                for p in periode {
                                    tr { key: "{p.id}",
                                        td {"{p.bezeichnung}"}
                                        td { class:"aktion", Delete{periode_resource: periode_resource, id:p.id} }
                                    }
                                }
                               
                            }
                        }
                    }
                },

                Some(Err(e)) => rsx! {
                    div {
                        style: "color: read; font-size:bold;",
                        "Fehler beim Laden der Periode: {e}"
                    }
                },
                None => rsx! {
                    div{"Daten sind am Laden..."}
                }
            }
        }
    }
}