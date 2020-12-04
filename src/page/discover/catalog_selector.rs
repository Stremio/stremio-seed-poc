use crate::entity::multi_select;
use itertools::Itertools;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::state_types::{CatalogFiltered, CatalogEntry};
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
    multi_select::view("Select catalog", items)
}

// ------ ------
//    Items
// ------ ------

pub fn items<Ms: 'static>(
    catalog: &CatalogFiltered<MetaPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    let catalog_entries = &catalog.catalogs;
    let selected_req = &catalog.selected;

    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new(),
    };

    let catalog_entries = catalog_entries
        .iter()
        .filter(|catalog_entry| catalog_entry.load.path.type_name == selected_req.path.type_name);

    let catalog_groups = catalog_entries.group_by(|catalog_entry| &catalog_entry.addon_name);

    catalog_groups
        .into_iter()
        .flat_map(|(_addon_name, catalog_entries)| {
            catalog_entries
                .map(|catalog_entry| {
                    let res_req = catalog_entry.load.clone(); 
                    multi_select::Item {
                        title: catalog_entry.name.clone(),
                        selected: catalog_entry.is_selected,
                        on_click: Rc::new(move || {
                            send_res_req_msg(res_req.clone())
                        }),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
