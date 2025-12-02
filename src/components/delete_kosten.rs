use dioxus::prelude::*;
use crate::icons::{Icon, mdi_light};

use crate::backend::server_functions::kosten_fns::delete_kosten;
use crate::backend::models::kosten::Kosten;

#[component]
pub fn Delete(mut kosten_resource: Resource<Result<Vec<Kosten>, ServerFnError>>, id:i64) -> Element {
    rsx!{
        button { 
            onclick: move |_| async move {
                match delete_kosten(id).await {
                    Ok(_) => {
                        kosten_resource.restart();
                    }
                    Err(e) => {}
                }
            },
            Icon{data:mdi_light::Delete}
        }
    }
}