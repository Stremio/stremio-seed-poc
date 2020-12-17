use crate::entity::multi_select;
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



// use super::{BASE, MY_ITEM_ID, RESOURCE, TYPE_ALL};
// use crate::entity::multi_select;
// use seed::prelude::*;
// use std::collections::BTreeSet;
// use std::fmt::Debug;
// use stremio_core::state_types::CatalogEntry;
// use stremio_core::types::addons::{Descriptor, ResourceRef, ResourceRequest};

// ------ ------
//     View
// ------ ------

// pub fn view<T: Clone>(model: &Model, groups: &[multi_select::Group<T>]) -> Node<Msg> {
//     multi_select::view(&model.0, groups).map_msg(Msg)
// }

// ------ ------
//  Conversion
// ------ ------

// pub fn groups(
//     catalog_entries: &[CatalogEntry],
//     selected_req: &Option<ResourceRequest>,
//     installed_addons: &[Descriptor],
// ) -> Vec<multi_select::Group<CatalogEntry>> {
//     let selected_req = match selected_req {
//         Some(selected_req) => selected_req,
//         None => return Vec::new(),
//     };

//     let mut installed_addon_types = BTreeSet::new();
//     installed_addon_types.insert(TYPE_ALL);
//     for type_ in installed_addons
//         .iter()
//         .flat_map(|addon| &addon.manifest.types)
//     {
//         installed_addon_types.insert(type_);
//     }

//     let my_catalog_entries = installed_addon_types
//         .into_iter()
//         .map(|installed_addon_type| CatalogEntry {
//             name: "my".to_owned(),
//             is_selected: selected_req.path.id == MY_ITEM_ID
//                 && selected_req.path.type_name == installed_addon_type,
//             addon_name: "my_addon".to_owned(),
//             load: ResourceRequest::new(
//                 BASE,
//                 ResourceRef::without_extra(RESOURCE, installed_addon_type, MY_ITEM_ID),
//             ),
//         })
//         .collect::<Vec<_>>();

//     let items = catalog_entries
//         .iter()
//         .chain(my_catalog_entries.iter())
//         .filter_map(|catalog_entry| {
//             if catalog_entry.load.path.id == selected_req.path.id {
//                 Some(multi_select::GroupItem {
//                     id: catalog_entry.load.path.type_name.clone(),
//                     label: catalog_entry.load.path.type_name.clone(),
//                     selected: catalog_entry.is_selected,
//                     value: catalog_entry.clone(),
//                 })
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();

//     vec![multi_select::Group {
//         id: "default".to_owned(),
//         label: None,
//         items,
//         limit: 1,
//         required: true,
//     }]
// }

// pub fn resource_request(
//     groups_with_selected_items: Vec<multi_select::Group<CatalogEntry>>,
// ) -> ResourceRequest {
//     groups_with_selected_items
//         .into_iter()
//         .next()
//         .expect("type selector's group `default`")
//         .items
//         .into_iter()
//         .next()
//         .expect("type selector's selected item")
//         .value
//         .load
// }
