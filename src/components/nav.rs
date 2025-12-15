use dioxus::prelude::*;



#[component]
pub fn Nav() -> Element {
    rsx!{ 
        div { class:"nav",
            h3 {
                "Navigation"
            }

            details {  
                summary { "Finanzen" }
                p { Link { to: "/buchung", "Budgetübersicht" } }
                p { Link { to: "/jahresuebersicht", "Jahresübersicht" } }
                p { Link { to: "/abo/add", "Abo hinzufügen" } }
            }
            details {  
                summary { "Aufgaben" }
                p { Link { to: "/buchung", "Budgetübersicht" } }
                p { Link { to: "/jahresuebersicht", "Jahresübersicht" } }
                p { Link { to: "/abo/add", "Abo hinzufügen" } }
            }

            details {  
                summary { "Dokumente" }
                p { Link { to: "/buchung", "Budgetübersicht" } }
                p { Link { to: "/jahresuebersicht", "Jahresübersicht" } }
                p { Link { to: "/abo/add", "Abo hinzufügen" } }
            }
            details{ 
                summary { "Einstellungen" }
                p { Link { to: "/periode", "Perioden" } }
            }

  
        }  
    }
}