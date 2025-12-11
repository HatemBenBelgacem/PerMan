use dioxus::prelude::*;
use crate::icons::{Icon, mdi_light};

use crate::backend::server_functions::periode_fns::delete_periode;
use crate::backend::models::periode::Periode;

#[component]
pub fn Delete(mut periode_resource: Resource<Result<Vec<Periode>, ServerFnError>>, id:i64) -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                match delete_periode(id).await {
                    Ok(_) => {
                        periode_resource.restart();
                    }
                    Err(e) => ()
                }
            },
            Icon{data:mdi_light::Delete}
        }
    }
}