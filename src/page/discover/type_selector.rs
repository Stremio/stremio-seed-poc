use crate::entity::multi_select;
use seed::prelude::*;
use stremio_core::state_types::TypeEntry;
use stremio_core::types::addons::ResourceRequest;

// ------ ------
//     Model
// ------ ------

pub struct Model(multi_select::Model);

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model(multi_select::init("type-selector"))
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub struct Msg(multi_select::Msg);

pub fn update<T: 'static, ParentMsg>(
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

pub fn groups(type_entries: &[TypeEntry]) -> Vec<multi_select::Group<TypeEntry>> {
    let items = type_entries
        .iter()
        .map(|type_entry| multi_select::GroupItem {
            id: type_entry.type_name.clone(),
            label: type_entry.type_name.clone(),
            selected: type_entry.is_selected,
            value: type_entry.clone(),
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
    groups_with_selected_items: Vec<multi_select::Group<TypeEntry>>,
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
