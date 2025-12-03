use dioxus::prelude::*;
use crate::icons::{Icon, mdi_light};

use crate::backend::server_functions::buchung_fns::delete_buchung;
use crate::backend::models::buchung::Buchung;

#[component]
pub fn Delete(mut buchung_resource: Resource<Result<Vec<Buchung>, ServerFnError>>, id:i64) -> Element {
    rsx!{
        button { 
            onclick: move |_| async move {
                match delete_buchung(id).await {
                    Ok(_) => {
                        buchung_resource.restart();
                    }
                    Err(e) => {}
                }
            },
            Icon{data:mdi_light::Delete}
        }
    }
}