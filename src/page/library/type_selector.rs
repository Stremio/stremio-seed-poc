use crate::multi_select;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::library_with_filters::{LibraryWithFilters, LibraryRequest};
use seed_hooks::{*, topo::nested as view};

// ------ ------
//     View
// ------ ------

#[view]
pub fn view<Ms: 'static, F>(
    library: &LibraryWithFilters<F>,
    send_library_request_msg: impl Fn(LibraryRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(library, send_library_request_msg);
    multi_select::view("Select type", items, false)
}

pub fn items<Ms: 'static, F>(
    library: &LibraryWithFilters<F>,
    send_library_request_msg: impl Fn(LibraryRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    library
        .selectable
        .types
        .iter()
        .map(|type_| {
            let library_request = type_.request.clone();
            multi_select::Item {
                title: type_.r#type.clone().unwrap_or_else(|| "All".to_owned()),
                selected: type_.selected,
                on_click: Rc::new(move || {
                    send_library_request_msg(library_request.clone())
                }),
            }
        })
        .collect()
}
