use crate::entity::multi_select;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::state_types::{CatalogFiltered, TypeEntry};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::ResourceRequest;

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(
    catalog: &CatalogFiltered<MetaPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(catalog, send_res_req_msg);
    multi_select::view("Select type", items)
}

pub fn items<Ms: 'static>(
    catalog: &CatalogFiltered<MetaPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    catalog.types
        .iter()
        .map(|type_entry| {
            let res_req = type_entry.load.clone();
            multi_select::Item {
                title: type_entry.type_name.clone(),
                selected: type_entry.is_selected,
                on_click: Rc::new(move || {
                    send_res_req_msg(res_req.clone())
                }),
            }
        })
        .collect()
}
