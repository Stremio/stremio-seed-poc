use crate::entity::multi_select;
use crate::page::discover::ExtraPropOption;
use seed::prelude::*;
use stremio_core::types::addons::{ManifestExtraProp, ResourceRequest};

// ------ ------
//     Model
// ------ ------

pub struct Model(multi_select::Model);

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model(multi_select::init())
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

pub fn groups(
    extra_props: &[ManifestExtraProp],
    selected_req: &Option<ResourceRequest>,
) -> Vec<multi_select::Group<ExtraPropOption>> {
    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new(),
    };

    extra_props
        .into_iter()
        .map(|extra_prop| {
            let group_id = extra_prop.name.clone();

            let items = if let Some(options) = &extra_prop.options {
                options
                    .iter()
                    .map(|option| {
                        let item_id = option.clone();
                        multi_select::GroupItem {
                            id: item_id.clone(),
                            label: option.clone(),
                            selected: selected_req
                                .path
                                .extra
                                .contains(&(group_id.clone(), item_id.clone())),
                            value: option.clone(),
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

            multi_select::Group {
                id: group_id,
                label: Some(extra_prop.name.clone()),
                // @TODO OptionsLimit?
                limit: extra_prop.options_limit.0,
                required: extra_prop.is_required,
                items,
            }
        })
        .collect()
}

pub fn resource_request(
    groups_with_selected_items: Vec<multi_select::Group<ExtraPropOption>>,
    selected_req: &Option<ResourceRequest>,
) -> Option<ResourceRequest> {
    let selected_pairs = groups_with_selected_items
        .into_iter()
        .flat_map(|group| {
            let group_id = group.id.clone();
            let pairs = group
                .items
                .into_iter()
                .map(|item| (group_id.clone(), item.value))
                .collect::<Vec<_>>();
            pairs
        })
        .collect::<Vec<_>>();

    selected_req.as_ref().map(|selected_req| {
        let mut req = selected_req.clone();
        req.path.extra = selected_pairs;
        req
    })
}
