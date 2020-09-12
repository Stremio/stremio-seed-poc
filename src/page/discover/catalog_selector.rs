use crate::entity::multi_select;
use itertools::Itertools;
use seed::prelude::*;
use std::fmt::Debug;
use stremio_core::state_types::CatalogEntry;
use stremio_core::types::addons::ResourceRequest;

// ------ ------
//     Model
// ------ ------

pub struct Model(multi_select::Model);

// ------ ------
//     Init
// ------ ------

pub const fn init() -> Model {
    Model(multi_select::init("catalog-selector"))
}

// ------ ------
//    Update
// ------ ------

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
    multi_select::view(&model.0, groups).map_msg(Msg)
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

    let catalog_entries = catalog_entries
        .iter()
        .filter(|catalog_entry| catalog_entry.load.path.type_name == selected_req.path.type_name);

    let catalog_groups = catalog_entries.group_by(|catalog_entry| &catalog_entry.addon_name);

    catalog_groups
        .into_iter()
        .map(|(addon_name, catalog_entries)| {
            let items = catalog_entries
                .map(|catalog_entry| multi_select::GroupItem {
                    id: catalog_entry.name.clone(),
                    label: catalog_entry.name.clone(),
                    selected: catalog_entry.is_selected,
                    value: catalog_entry.clone(),
                })
                .collect::<Vec<_>>();

            multi_select::Group {
                id: "default".to_owned(),
                label: Some(addon_name.clone()),
                limit: 1,
                required: true,
                items,
            }
        })
        .collect()
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
