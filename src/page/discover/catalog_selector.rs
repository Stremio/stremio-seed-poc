use crate::entity::multi_select;
use itertools::Itertools;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::common::Loadable;
use stremio_core::types::resource::MetaItemPreview;
use stremio_core::types::addon::ResourceRequest;
use seed_hooks::{*, topo::nested as view};


// ------ ------
//     View
// ------ ------

#[view]
pub fn view<Ms: 'static>(
    catalog: &CatalogWithFilters<MetaItemPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(catalog, send_res_req_msg);
    multi_select::view("Select catalog", items, true)
}

// ------ ------
//    Items
// ------ ------

pub fn items<Ms: 'static>(
    catalog: &CatalogWithFilters<MetaItemPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    // let selected_request = if let Some(selected_request) = catalog.selected {
    //     selected_request
    // } else {
    //     return Vec::new()
    // };

    catalog
        .selectable
        .catalogs
        .iter()
        .map(|catalog| {
            let res_req = catalog.request.clone(); 
            multi_select::Item {
                title: catalog.catalog.clone(),
                selected: catalog.selected,
                on_click: Rc::new(move || {
                    send_res_req_msg(res_req.clone())
                }),
            }
        })
        .collect()
}
