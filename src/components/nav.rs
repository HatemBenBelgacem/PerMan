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
                    
                p { Link { to: "/buchung", "Buchungsübersicht" } }
                p { Link { to: "/buchung", "Monatsübersicht" } }
                


            }

  
        }  
    }
}