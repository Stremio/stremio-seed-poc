use crate::multi_select;
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::installed_addons_with_filters::InstalledAddonsWithFilters;
use stremio_core::types::addon::{ResourceRequest, DescriptorPreview};
use seed_hooks::{*, topo::nested as view};
use crate::page::addons::AddonRequest;

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
    multi_select::view("Select type", items, true)
}

pub fn items<Ms: 'static>(
    catalog: &CatalogWithFilters<DescriptorPreview>,
    installed_addons: &InstalledAddonsWithFilters,
    send_res_req_msg: impl Fn(AddonRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    let installed_addons_selected = installed_addons.selected.is_some();

    if installed_addons_selected {
        installed_addons
            .selectable
            .types
            .iter()
            .map(|type_| {
                let installed_addon_request = type_.request.clone();
                multi_select::Item {
                    title: type_.r#type.clone().unwrap_or("All".to_owned()),
                    selected: type_.selected,
                    on_click: Rc::new(move || {
                        send_res_req_msg(AddonRequest::Installed(installed_addon_request.clone()))
                    }),
                }
            })
            .collect()
    } else {
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
                        send_res_req_msg(AddonRequest::Remote(res_req.clone()))
                    }),
                }
            })
            .collect()
    }
}
