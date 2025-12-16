use dioxus::prelude::*;
use chrono::Months;
use crate::backend::server_functions::abo_fns::{list_abo, loesche_abo};

#[component]
pub fn AboListe() -> Element {
    let abo_resource = use_resource(move || async move {list_abo().await});

    rsx! {
        div {
            class:"functions",
            Link {
                class:"btn",
                to: "/abo/add", "Neues Abo",
            }
            Link {
                class:"btn",
                to: "/", "Zurück"
            }
        }

        div {
            class:"liste",
            match &*abo_resource.read_unchecked(){
                Some(Ok(abo)) => rsx! {
                    if abo.is_empty() {
                        div{"Keine Abos"}
                    } else {
                        table {
                            thead{
                                tr{
                                    th{"Bezeichnung"}
                                    th{"Beginn"}
                                    th{"Ende"}
                                    th{"Dauer"}
                                    th{"Kündigungsfrist"}
                                    th{"Aktion"}
                                }
                            }
                            tbody{
                                for a in abo {
                                    tr{
                                        key:"a.id",
                                        td{"{a.bezeichnung}"}
                                        td{"{a.beginn.format(\"%d.%m.%Y\")}"}
                                        td { 
                                            "{a.beginn.checked_add_months(Months::new(a.dauer as u32))
                                                .unwrap()
                                                .format(\"%d.%m.%Y\")}" 
                                            }
                                        td{"{a.dauer}"}
                                        td{"{a.knd_frist}"}
                                        td { class:"aktion" } 
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div{
                        "Fehler beim Lader der Abos: {e}"
                    }
                },
                None => rsx!{
                    div{
                        "Lade Abos..."
                    }
                }
            }
        }
    }
}

