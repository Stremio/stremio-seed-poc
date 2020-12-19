use crate::multi_select;
use itertools::Itertools;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::installed_addons_with_filters::{InstalledAddonsWithFilters, InstalledAddonsRequest};
use stremio_core::models::common::Loadable;
use stremio_core::types::addon::{ResourceRequest, DescriptorPreview};
use seed_hooks::{*, topo::nested as view};
use crate::page::addons::AddonRequest;
use std::iter;

// ------ ------
//     View
// ------ ------

#[view]
pub fn view<Ms: 'static>(
    catalog: &CatalogWithFilters<DescriptorPreview>,
    installed_addons: &InstalledAddonsWithFilters,
    send_res_req_msg: impl Fn(AddonRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(catalog, installed_addons, send_res_req_msg);
    multi_select::view("Select catalog", items, true)
}

// ------ ------
//    Items
// ------ ------

pub fn items<Ms: 'static>(
    catalog: &CatalogWithFilters<DescriptorPreview>,
    installed_addons: &InstalledAddonsWithFilters,
    send_res_req_msg: impl Fn(AddonRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    catalog
        .selectable
        .catalogs
        .iter()
        .map(|catalog_item| {
            let res_req = catalog_item.request.clone(); 
            multi_select::Item {
                title: catalog_item.catalog.clone(),
                selected: catalog_item.selected && catalog.selected.is_some(),
                on_click: Rc::new(move || {
                    send_res_req_msg(AddonRequest::Remote(res_req.clone()))
                }),
            }
        })
        .chain(iter::once(multi_select::Item {
            title: "Installed".to_owned(),
            selected: installed_addons.selected.is_some(),
            on_click: Rc::new(move || send_res_req_msg(AddonRequest::default())),
        }))
        .collect()
}




