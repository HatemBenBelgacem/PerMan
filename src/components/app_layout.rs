use dioxus::{prelude::*};

use crate::components::{header::Header, nav::Nav};
use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    let is_logged_in = use_context::<Signal<bool>>();
    let nav = use_navigator();

    if !*is_logged_in.read() {
        use_effect(move || {
            nav.replace(Route::LoginPage {});
        });

        return rsx!{ div {"Bitte Warten..."}};
    }
    
    
    
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