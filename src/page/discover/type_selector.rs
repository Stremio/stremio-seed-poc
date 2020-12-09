use crate::entity::multi_select;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
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
    multi_select::view("Select type", items, false)
}

pub fn items<Ms: 'static>(
    catalog: &CatalogWithFilters<MetaItemPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    catalog
        .selectable
        .types
        .iter()
        .map(|type_| {
            let res_req = type_.request.clone();
            multi_select::Item {
                title: type_.r#type.clone(),
                selected: type_.selected,
                on_click: Rc::new(move || {
                    send_res_req_msg(res_req.clone())
                }),
            }
        })
        .collect()
}
