use crate::{entity::multi_select, page::discover::ExtraPropOption};
use seed::{*, prelude::*};
use std::fmt::Debug;
use std::rc::Rc;
use stremio_core::state_types::CatalogFiltered;
use stremio_core::types::addons::{ManifestExtraProp, ResourceRequest};
use stremio_core::types::MetaPreview;

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(
    catalog: &CatalogFiltered<MetaPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Node<Ms> {
    let items = items(catalog, send_res_req_msg);
    multi_select::view("Select genre", items)
}

// ------ ------
//    Items
// ------ ------

pub fn items<Ms: 'static>(
    catalog: &CatalogFiltered<MetaPreview>,
    send_res_req_msg: impl Fn(ResourceRequest) -> Ms + 'static + Copy,
) -> Vec<multi_select::Item<Ms>> {
    let extra_props = &catalog.selectable_extra;
    let selected_req = &catalog.selected;

    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new(),
    };

    extra_props
        .iter()
        .map(|extra_prop| {
            let group_id = extra_prop.name.clone();

            if let Some(options) = &extra_prop.options {
                options
                    .iter()
                    .map(|option| {
                        let item_id = option.clone();
                        let res_req = catalog_entry.load.clone();

                        multi_select::Item {
                            title: option.clone(),
                            selected: selected_req
                                .path
                                .extra
                                .contains(&(group_id.clone(), item_id)),
                            on_click: Rc::new(move || {
                                send_res_req_msg(res_req.clone())
                            }),
                        }
                    })
                    .collect::<Vec<multi_select::Item<Ms>>>()
            } else {
                Vec::new()
            }
        })
        .collect()
}

// ------ ------
//     View
// ------ ------

// pub fn view<T: Clone>(model: &Model, groups: &[multi_select::Group<T>]) -> Node<Msg> {
//     //multi_select::view(&model.0, groups).map_msg(Msg)
//     empty![]
// }

// // ------ ------
// //  Conversion
// // ------ ------

// pub fn groups(
//     extra_props: &[ManifestExtraProp],
//     selected_req: &Option<ResourceRequest>,
// ) -> Vec<multi_select::Group<ExtraPropOption>> {
//     let selected_req = match selected_req {
//         Some(selected_req) => selected_req,
//         None => return Vec::new(),
//     };

//     extra_props
//         .iter()
//         .map(|extra_prop| {
//             let group_id = extra_prop.name.clone();

//             let items = if let Some(options) = &extra_prop.options {
//                 options
//                     .iter()
//                     .map(|option| {
//                         let item_id = option.clone();
//                         multi_select::GroupItem {
//                             id: item_id.clone(),
//                             label: option.clone(),
//                             selected: selected_req
//                                 .path
//                                 .extra
//                                 .contains(&(group_id.clone(), item_id)),
//                             value: option.clone(),
//                         }
//                     })
//                     .collect()
//             } else {
//                 Vec::new()
//             };

//             multi_select::Group {
//                 id: group_id,
//                 label: Some(extra_prop.name.clone()),
//                 // @TODO OptionsLimit?
//                 limit: extra_prop.options_limit.0,
//                 required: extra_prop.is_required,
//                 items,
//             }
//         })
//         .collect()
// }

// pub fn resource_request(
//     groups_with_selected_items: Vec<multi_select::Group<ExtraPropOption>>,
//     selected_req: &Option<ResourceRequest>,
// ) -> Option<ResourceRequest> {
//     let selected_pairs = groups_with_selected_items
//         .into_iter()
//         .flat_map(|group| {
//             let group_id = group.id.clone();
//             group
//                 .items
//                 .into_iter()
//                 .map(|item| (group_id.clone(), item.value))
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();

//     selected_req.as_ref().map(|selected_req| {
//         let mut req = selected_req.clone();
//         req.path.extra = selected_pairs;
//         req
//     })
// }
