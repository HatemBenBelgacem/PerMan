use dioxus::prelude::*;



#[component]
pub fn Nav() -> Element {
    rsx!{
        h3 { class:"nav",
            "Navigation"}

        li { 
            ul {
                class:"nav", 
                Link { to: "/kosten", "Kosten" }
            }
        }    
       
    }
}