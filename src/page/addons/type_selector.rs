use super::{BASE, MY_ITEM_ID, RESOURCE, TYPE_ALL};
use crate::{entity::multi_select, GMsg};
use seed::prelude::*;
use std::collections::BTreeSet;
use std::fmt::Debug;
use stremio_core::state_types::CatalogEntry;
use stremio_core::types::addons::{Descriptor, ResourceRef, ResourceRequest};

// ------ ------
//     Model
// ------ ------

pub struct Model(multi_select::Model);

// ------ ------
//     Init
// ------ ------

pub const fn init() -> Model {
    Model(multi_select::init("type-selector"))
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub struct Msg(multi_select::Msg);

pub fn update<T: 'static + Debug, ParentMsg>(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>,
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
    multi_select::view(&model.0, groups).map_msg(Msg)
}

// ------ ------
//  Conversion
// ------ ------

pub fn groups(
    catalog_entries: &[CatalogEntry],
    selected_req: &Option<ResourceRequest>,
    installed_addons: &[Descriptor],
) -> Vec<multi_select::Group<CatalogEntry>> {
    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new(),
    };

    let mut installed_addon_types = BTreeSet::new();
    installed_addon_types.insert(TYPE_ALL);
    for type_ in installed_addons
        .iter()
        .flat_map(|addon| &addon.manifest.types)
    {
        installed_addon_types.insert(type_);
    }

    let my_catalog_entries = installed_addon_types
        .into_iter()
        .map(|installed_addon_type| CatalogEntry {
            name: "my".to_owned(),
            is_selected: selected_req.path.id == MY_ITEM_ID
                && selected_req.path.type_name == installed_addon_type,
            addon_name: "my_addon".to_owned(),
            load: ResourceRequest::new(
                BASE,
                ResourceRef::without_extra(RESOURCE, installed_addon_type, MY_ITEM_ID),
            ),
        })
        .collect::<Vec<_>>();

    let items = catalog_entries
        .iter()
        .chain(my_catalog_entries.iter())
        .filter_map(|catalog_entry| {
            if catalog_entry.load.path.id == selected_req.path.id {
                Some(multi_select::GroupItem {
                    id: catalog_entry.load.path.type_name.clone(),
                    label: catalog_entry.load.path.type_name.clone(),
                    selected: catalog_entry.is_selected,
                    value: catalog_entry.clone(),
                })
            } else {
                None
            }
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
        .expect("type selector's group `default`")
        .items
        .into_iter()
        .next()
        .expect("type selector's selected item")
        .value
        .load
}
