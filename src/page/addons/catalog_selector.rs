use super::{BASE, MY_ITEM_ID, RESOURCE, TYPE_ALL};
use crate::entity::multi_select;
use itertools::Itertools;
use seed::prelude::*;
use std::fmt::Debug;
use std::iter;
use stremio_core::state_types::CatalogEntry;
use stremio_core::types::addons::{ResourceRef, ResourceRequest};

// ------ ------
//     Model
// ------ ------

pub struct Model(multi_select::Model);

// ------ ------
//     Init
// ------ ------

pub const fn init() -> Model {
    Model(multi_select::init("category-selector"))
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub struct Msg(multi_select::Msg);

pub fn update<T: 'static + Debug, ParentMsg>(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
    groups: Vec<multi_select::Group<T>>,
    on_change: impl FnOnce(Vec<multi_select::Group<T>>) -> ParentMsg,
) -> Option<ParentMsg> {
    multi_select::update(
        msg.0,
        &mut model.0,
        &mut orders.proxy(Msg),
        groups,
        on_change,
    )
}

// ------ ------
//     View
// ------ ------

pub fn view<T: Clone>(model: &Model, groups: &[multi_select::Group<T>]) -> Node<Msg> {
    multi_select::view(&model.0, groups).map_message(Msg)
}

// ------ ------
//  Conversion
// ------ ------

pub fn groups(
    catalog_entries: &[CatalogEntry],
    selected_req: &Option<ResourceRequest>,
) -> Vec<multi_select::Group<CatalogEntry>> {
    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new(),
    };

    let my_catalog_entry = CatalogEntry {
        name: "my".to_owned(),
        is_selected: selected_req.path.id == MY_ITEM_ID,
        addon_name: "my_addon".to_owned(),
        load: ResourceRequest::new(
            BASE,
            ResourceRef::without_extra(RESOURCE, TYPE_ALL, MY_ITEM_ID),
        ),
    };

    let items = catalog_entries
        .iter()
        .chain(iter::once(&my_catalog_entry))
        .group_by(|catalog_entry| &catalog_entry.name)
        .into_iter()
        .flat_map(|(_, catalog_entries)| {
            catalog_entries
                .sorted_by_key(|catalog_entry| !catalog_entry.is_selected)
                .unique_by(|catalog_entry| &catalog_entry.name)
        })
        .map(|catalog_entry| multi_select::GroupItem {
            id: catalog_entry.name.clone(),
            label: catalog_entry.name.clone(),
            selected: catalog_entry.is_selected,
            value: catalog_entry.clone(),
        })
        .collect::<Vec<_>>();

    vec![multi_select::Group {
        id: "default".to_owned(),
        label: None,
        items,
        limit: 1,
        required: true,
    }]
}

pub fn resource_request(
    groups_with_selected_items: Vec<multi_select::Group<CatalogEntry>>,
) -> ResourceRequest {
    groups_with_selected_items
        .into_iter()
        .next()
        .expect("catalog selector's group `default`")
        .items
        .into_iter()
        .next()
        .expect("catalog selector's selected item")
        .value
        .load
}
