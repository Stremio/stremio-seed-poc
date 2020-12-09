// use super::{BASE, MY_ITEM_ID, RESOURCE, TYPE_ALL};
use crate::entity::multi_select;
use itertools::Itertools;
use seed::prelude::*;
use std::fmt::Debug;
use std::iter;
// use stremio_core::state_types::CatalogEntry;
// use stremio_core::types::addons::{ResourceRef, ResourceRequest};

// ------ ------
//     View
// ------ ------

// pub fn view<T>(items: Vec<multi_select::Item<CatalogEntry>>) -> Node<Msg> {
//     multi_select::view(items).map_msg(Msg)
// }

// // ------ ------
// //  Conversion
// // ------ ------

// pub fn items(
//     catalog_entries: &[CatalogEntry],
//     selected_req: &Option<ResourceRequest>,
// ) -> Vec<multi_select::Item<CatalogEntry>> {
//     let selected_req = match selected_req {
//         Some(selected_req) => selected_req,
//         None => return Vec::new(),
//     };

//     let my_catalog_entry = CatalogEntry {
//         name: "my".to_owned(),
//         is_selected: selected_req.path.id == MY_ITEM_ID,
//         addon_name: "my_addon".to_owned(),
//         load: ResourceRequest::new(
//             BASE,
//             ResourceRef::without_extra(RESOURCE, TYPE_ALL, MY_ITEM_ID),
//         ),
//     };

//     catalog_entries
//         .iter()
//         .chain(iter::once(&my_catalog_entry))
//         .group_by(|catalog_entry| &catalog_entry.name)
//         .into_iter()
//         .flat_map(|(_, catalog_entries)| {
//             catalog_entries
//                 .sorted_by_key(|catalog_entry| !catalog_entry.is_selected)
//                 .unique_by(|catalog_entry| &catalog_entry.name)
//         })
//         .map(|catalog_entry| multi_select::Item {
//             title: catalog_entry.name.clone(),
//             selected: catalog_entry.is_selected,
//             value: catalog_entry.clone(),
//         })
//         .collect::<Vec<_>>()
// }

// pub fn resource_request(
//     items: Vec<multi_select::Item<CatalogEntry>>,
// ) -> ResourceRequest {
//     items
//         .into_iter()
//         .find(|item| item.selected)
//         .expect("selected item")
//         .value
//         .load
// }
