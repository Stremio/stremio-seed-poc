use seed::{prelude::*, *};
use itertools::Itertools;
use std::rc::Rc;
use stremio_core::state_types::{Action, ActionLoad, Loadable, TypeEntry, CatalogEntry, CatalogError, Msg as CoreMsg, Update};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::{ResourceRequest, ManifestExtraProp, OptionsLimit};
use crate::{default_resource_request, entity::multi_select, SharedModel, Route};
use futures::future::Future;
use std::convert::TryFrom;

type MetaPreviewId = String;
type ExtraPropOption = String;

// ------ ------
//     Model
// ------ ------

pub struct Model {
    shared: SharedModel,
    resource_request: ResourceRequest,
    selected_meta_preview_id: Option<MetaPreviewId>,
    type_selector_model: multi_select::Model,
    catalog_selector_model: multi_select::Model,
    extra_prop_selector_model: multi_select::Model,
}

impl From<Model> for SharedModel {
    fn from(model: Model) -> Self {
        model.shared
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(shared: SharedModel, resource_request: ResourceRequest, orders: &mut impl Orders<Msg>) -> Model {
    orders.send_msg(
        Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(resource_request.clone())))))
    );

    Model {
        type_selector_model: multi_select::init(),
        catalog_selector_model: multi_select::init(),
        extra_prop_selector_model: multi_select::init(),
        resource_request,
        selected_meta_preview_id: None,
        shared,
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    MetaPreviewClicked(MetaPreviewId),
    TypeSelectorMsg(multi_select::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<TypeEntry>>),
    CatalogSelectorMsg(multi_select::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    ExtraPropSelectorMsg(multi_select::Msg),
    ExtraPropSelectorChanged(Vec<multi_select::Group<ExtraPropOption>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Core(core_msg) => {
            let fx = model.shared.core.update(&core_msg);

            if !fx.has_changed {
                orders.skip();
            }

            for cmd in fx.effects {
                let cmd = cmd
                    .map(|core_msg| Msg::Core(Rc::new(core_msg)))
                    .map_err(|core_msg| Msg::CoreError(Rc::new(core_msg)));
                orders.perform_cmd(cmd);
            }
        },
        Msg::CoreError(core_error) => log!("core_error", core_error),
        Msg::MetaPreviewClicked(meta_preview_id) => {
            if let Some(selected_meta_preview_id) = &model.selected_meta_preview_id {
                if selected_meta_preview_id == &meta_preview_id {
                    // @TODO go to player
                }
            }
            model.selected_meta_preview_id = Some(meta_preview_id);
        },
        Msg::TypeSelectorMsg(msg) => {
            let msg_to_parent = multi_select::update(
                msg,
                &mut model.type_selector_model,
                &mut orders.proxy(Msg::TypeSelectorMsg),
                type_selector_groups(&model.shared.core.catalog.types),
                on_type_selector_change
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::TypeSelectorChanged(groups_with_selected_items) => {
            let req = groups_with_selected_items
                .first()
                .expect("type selector's group `default`")
                .items
                .first()
                .expect("type selector's selected item")
                .value
                .load
                .clone();

            push_route(req.clone());
            orders.send_msg(
                Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))
            );
        },
        Msg::CatalogSelectorMsg(msg) => {
            let msg_to_parent = multi_select::update(
                msg,
                &mut model.catalog_selector_model,
                &mut orders.proxy(Msg::CatalogSelectorMsg),
                catalog_selector_groups(&model.shared.core.catalog.catalogs, &model.shared.core.catalog.selected),
                on_catalog_selector_change
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::CatalogSelectorChanged(groups_with_selected_items) => {
            let req = groups_with_selected_items
                .first()
                .expect("catalog selector's group `default`")
                .items
                .first()
                .expect("catalog selector's selected item")
                .value
                .load
                .clone();

            push_route(req.clone());
            orders.send_msg(
                Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))
            );
        },
        Msg::ExtraPropSelectorMsg(msg) => {
            let msg_to_parent = multi_select::update(
                msg,
                &mut model.extra_prop_selector_model,
                &mut orders.proxy(Msg::ExtraPropSelectorMsg),
                extra_prop_selector_groups(&model.shared.core.catalog.selectable_extra, &model.shared.core.catalog.selected),
                on_extra_prop_selector_change
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::ExtraPropSelectorChanged(groups_with_selected_items) => {
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
                }).collect::<Vec<_>>();

            if let Some(selected_req) = &model.shared.core.catalog.selected {
                let mut req = selected_req.clone();
                req.path.extra = selected_pairs;

                push_route(req.clone());
                orders.send_msg(
                    Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))
                );
            }
        },
    }
}

fn push_route(req: ResourceRequest) {
    let route = Route::Discover(req.clone());
    let url = Url::try_from(route.to_href().to_string()).expect("`Url` from `Route::Discover`");
    seed::push_route(url);
}

fn on_type_selector_change(groups_with_selected_items: Vec<multi_select::Group<TypeEntry>>) -> Msg {
    Msg::TypeSelectorChanged(groups_with_selected_items)
}

fn type_selector_groups(type_entries: &[TypeEntry]) -> Vec<multi_select::Group<TypeEntry>> {
    let items = type_entries.iter().map(|type_entry| {
        multi_select::GroupItem {
            id: type_entry.type_name.clone(),
            label: type_entry.type_name.clone(),
            selected: type_entry.is_selected,
            value: type_entry.clone()
        }
    }).collect::<Vec<_>>();

    vec![
        multi_select::Group {
            id: "default".to_owned(),
            label: None,
            items,
            limit: 1,
            required: true
        }
    ]
}

fn on_catalog_selector_change(groups_with_selected_items: Vec<multi_select::Group<CatalogEntry>>) -> Msg {
    Msg::CatalogSelectorChanged(groups_with_selected_items)
}

fn catalog_selector_groups(catalog_entries: &[CatalogEntry], selected_req: &Option<ResourceRequest>) -> Vec<multi_select::Group<CatalogEntry>> {
    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new()
    };

    let catalog_entries = catalog_entries
        .iter()
        .filter(|catalog_entry| &catalog_entry.load.path.type_name == &selected_req.path.type_name);

    let catalog_groups = catalog_entries.group_by(|catalog_entry| &catalog_entry.addon_name);

    catalog_groups
        .into_iter()
        .map(|(addon_name, catalog_entries)| {
            let items = catalog_entries.map(|catalog_entry| {
                multi_select::GroupItem {
                    id: catalog_entry.name.clone(),
                    label: catalog_entry.name.clone(),
                    selected: catalog_entry.is_selected,
                    value: catalog_entry.clone(),
                }
            }).collect::<Vec<_>>();

            multi_select::Group {
                id: "default".to_owned(),
                label: Some(addon_name.clone()),
                limit: 1,
                required: true,
                items
            }
        }).collect()
}

fn on_extra_prop_selector_change(groups_with_selected_items: Vec<multi_select::Group<ExtraPropOption>>) -> Msg {
    Msg::ExtraPropSelectorChanged(groups_with_selected_items)
}

fn extra_prop_selector_groups(extra_props: &[ManifestExtraProp], selected_req: &Option<ResourceRequest>) -> Vec<multi_select::Group<ExtraPropOption>> {
    let selected_req = match selected_req {
        Some(selected_req) => selected_req,
        None => return Vec::new()
    };

    extra_props
        .into_iter()
        .map(|extra_prop| {
            let group_id = extra_prop.name.clone();

            let items = if let Some(options) = &extra_prop.options {
                options.iter().map(|option| {
                    let item_id =  option.clone();
                    multi_select::GroupItem {
                        id: item_id.clone(),
                        label: option.clone(),
                        selected: selected_req.path.extra.contains(&(group_id.clone(), item_id.clone())),
                        value: option.clone(),
                    }
                }).collect()
            } else {
                Vec::new()
            };

            multi_select::Group {
                id: group_id,
                label: Some(extra_prop.name.clone()),
                // @TODO OptionsLimit?
                limit: extra_prop.options_limit.0,
                required: extra_prop.is_required,
                items
            }
        }).collect()
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        id!("discover"),
        div![
            // @TODO refactor
            multi_select::view(&model.type_selector_model, &type_selector_groups(&model.shared.core.catalog.types)).map_message(Msg::TypeSelectorMsg),
            multi_select::view(&model.catalog_selector_model, &catalog_selector_groups(&model.shared.core.catalog.catalogs, &model.shared.core.catalog.selected)).map_message(Msg::CatalogSelectorMsg),
            multi_select::view(&model.extra_prop_selector_model, &extra_prop_selector_groups(&model.shared.core.catalog.selectable_extra, &model.shared.core.catalog.selected)).map_message(Msg::ExtraPropSelectorMsg),
            view_reset_button(),
        ],
        div![
            id!("discover_holder"),
            style!{
                St::Top => px(350),
            },
            class![
                "holder",
            ],
            view_content(&model.shared.core.catalog.content, model.selected_meta_preview_id.as_ref()),
        ]
    ]
}

fn view_reset_button() -> Node<Msg> {
    a![
        style!{
            St::Padding => "3px 20px",
            St::Cursor => "pointer",
            St::Display => "inline-block",
        },
        attrs!{
            At::Href => Route::Discover(default_resource_request()).to_href()
        },
        "Reset",
    ]
}

fn view_content(content: &Loadable<Vec<MetaPreview>, CatalogError>, selected_meta_preview_id: Option<&MetaPreviewId>) -> Node<Msg> {
    match content {
        Loadable::Err(catalog_error) => h3![format!("{:#?}", catalog_error)],
        Loadable::Loading => h3!["Loading"],
        Loadable::Ready(meta_previews) if meta_previews.is_empty() => empty![],
        Loadable::Ready(meta_previews) => {
            ul![
                id!("discover-port"),
                class![
                    "items",
                    "scroll-pane",
                    "square",
                ],
                meta_previews.iter().map(|meta_preview| view_meta_preview(meta_preview, selected_meta_preview_id)).collect::<Vec<_>>()
            ]
        }
    }
}

fn view_meta_preview(meta_preview: &MetaPreview, selected_meta_preview_id: Option<&MetaPreviewId>) -> Node<Msg> {
    let default_poster = "https://www.stremio.com/images/add-on-money.png".to_owned();
    let poster = meta_preview.poster.as_ref().unwrap_or(&default_poster);
//    let poster_shape = meta_preview.poster_shape.to_str();

    let is_selected = match selected_meta_preview_id {
        Some(selected_meta_preview_id) => selected_meta_preview_id == &meta_preview.id,
        None => false,
    };

    li![
        class![
            "selected" => is_selected,
            "item"
        ],
        simple_ev(Ev::Click, Msg::MetaPreviewClicked(meta_preview.id.clone())),
        div![
            class![
                "name",
            ],
             meta_preview.name
        ],
        a![
            class![
                "thumb"
            ],
            style!{
                St::BackgroundImage => format!("url({})", poster)
            }
        ],
        div![
            class![
                "icon",
                "icon-ic_play",
                "button"
            ]
        ]
    ]
}
