use dioxus::{prelude::*};

use crate::components::{header::Header, nav::Nav};
use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx!{
        div { 
            class:"container",
            div {  
                class:"header",
                Header {  }
            }
            
            div {  
                class:"menu",
                Nav {  }
            }
            div {  
                class:"content",
                Outlet::<Route> {}
            }
        }
        
    }
    
}