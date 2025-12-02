use dioxus::prelude::*;



#[component]
pub fn Header() -> Element {
    rsx!{
        h3 { class:"header_title",
            Link { 
                to: "/", "Perman"
            }
        }
        
    }
    
}