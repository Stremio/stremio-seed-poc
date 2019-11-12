#![allow(clippy::needless_pass_by_value)]

use env_web::Env;
use seed::{prelude::*, App, *};
use stremio_core::state_types::{Action, ActionLoad, CatalogFiltered, Ctx, Loadable, Msg as CoreMsg, Update, TypeEntry, CatalogEntry, CatalogError};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::{ResourceRequest, ResourceRef, ManifestExtraProp};
use stremio_derive::Model;
use itertools::Itertools;
use futures::future::Future;
use std::rc::Rc;
use wasm_bindgen::JsCast;

// ------ ------
//     Model
// ------ ------

type MetaPreviewId = String;

#[derive(Default)]
struct Model {
    core: CoreModel,
    selected_meta_preview_id: Option<MetaPreviewId>,
}

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
}

// ------ ------
//     Init
// ------ ------

fn default_load() -> Msg {
    let req = ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("catalog", "movie", "top"),
    };
    Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    orders.send_msg(default_load());
    Init::default()
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
enum Msg {
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    MetaPreviewClicked(MetaPreviewId),
    SetSelectorValues,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let mut set_selector_values = true;
    match msg {
        Msg::Core(core_msg) => {
            let fx = model.core.update(&core_msg);
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
        // @TODO resolve properly
        Msg::SetSelectorValues => {
            set_selector_values = false;
            if let Some(selected) = &model.core.catalog.selected {
                if let Some(type_selector) = document().get_element_by_id("type_selector") {
                    let type_selector = type_selector.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                    type_selector.set_value(&selected.path.type_name);
                }
                if let Some(catalog_selector) = document().get_element_by_id("catalog_selector") {
                    let catalog_selector = catalog_selector.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                    catalog_selector.set_value(&selected.path.id);
                }
                if let Some(extra_prop_selector) = document().get_element_by_id("extra_prop_selector") {
                    let extra_prop_selector = extra_prop_selector.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                    let value = selected.path.extra.iter().map(|(_, value)| value).join(", ");
                    let value = if value.is_empty() {
                        "None"
                    } else {
                        &value
                    };
                    extra_prop_selector.set_value(value);
                }
            }
        }
    }
    if set_selector_values {
        orders
            .force_render_now()
            .send_msg(Msg::SetSelectorValues);
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
//    log!("TYPES:", model.catalog.types);
//    log!("CATALOGS:", model.catalog.catalogs);
//    log!("Extra:", model.catalog.selectable_extra);
//    log!("CONTENT", model.core.catalog.content);

    div![
        id!("discover"),
        div![
            view_type_selector(&model.core.catalog.types),
            view_catalog_selector(&model.core.catalog.catalogs, model.core.catalog.selected.as_ref()).unwrap_or_else(|| empty![]),
            view_extra_prop_selector(&model.core.catalog.selectable_extra, model.core.catalog.selected.as_ref()).unwrap_or_else(|| empty![]),
            view_reset_button(),
        ],
        div![
            id!("discover_holder"),
            style!{
                St::Top => px(40),
            },
            class![
                "holder",
            ],
            view_content(&model.core.catalog.content, model.selected_meta_preview_id.as_ref()),
        ]
    ]
}

fn view_reset_button() -> Node<Msg> {
    div![
        style!{
            St::Padding => "3px 20px",
            St::Cursor => "pointer",
            St::Display => "inline-block",
        },
        simple_ev(Ev::Click, default_load()),
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

fn view_extra_prop_selector(extra_props: &[ManifestExtraProp], selected_req: Option<&ResourceRequest>) -> Option<Node<Msg>> {
    let selected_req = selected_req?;

//    log!("extra_props", extra_props);
//    log!("selected_req", selected_req);

    let mut select_is_empty = true;

    let dummy_option_value = selected_req.path.extra.iter().map(|(_, value)| value).join(", ");
    let dummy_option = option![
        style!{
            St::Display => "none",
        },
        if dummy_option_value.is_empty() {
            "None"
        } else {
            &dummy_option_value
        }
    ];

    let select =
        select![
            id!("extra_prop_selector"),
            extra_props
                .iter()
                .map(|extra_prop| {
                    let option_nodes = view_extra_prop_selector_items(extra_prop, selected_req);
                    if !option_nodes.is_empty() {
                        select_is_empty = false;
                    }
                    optgroup![
                        attrs!{
                            At::Label => extra_prop.name
                        },
                        option_nodes,
                        dummy_option.clone(),
                    ]
                }).collect::<Vec<_>>()
        ];

    if select_is_empty {
        return None
    }
    Some(select)
}

// @TODO: don't hide on click? ...
fn view_extra_prop_selector_items(extra_prop: &ManifestExtraProp, selected_req: &ResourceRequest) -> Vec<Node<Msg>> {
    let options = match &extra_prop.options {
        Some(options) => options,
        None => return Vec::new()
    };

//    log!(extra_prop);

    options.iter().map(|option| {
        let option_is_selected = selected_req.path.extra
            .iter()
            .any(|(selected_name, selected_option)| {
                selected_name == &extra_prop.name && selected_option == option
            });

        let selected_count = selected_req.path.extra
            .iter()
            .filter(|(selected_name, _)| selected_name == &extra_prop.name)
            .count();

        let mut req = selected_req.clone();
        if option_is_selected {
            if !extra_prop.is_required || selected_count > 1 {
                req.path.extra.retain(|(selected_name, selected_option)| {
                    selected_name != &extra_prop.name || selected_option != option
                });
            }
        } else {
            // TODO: if selected_count == extra_prop.options_limit {
            if selected_count == 1 {
                let position = req.path.extra.iter().position(|(selected_name, _)| selected_name == &extra_prop.name);
                if let Some(position) = position {
                    req.path.extra.remove(position);
                }
            }
            req.path.extra.push((extra_prop.name.clone(), option.clone()));
        }

        option![
            attrs!{
                At::Selected => option_is_selected.as_at_value(),
            },
            simple_ev(Ev::Click, Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))),
            option,
            if option_is_selected { " ✓" } else { "" },
        ]
    }).collect()
}

fn view_type_selector(type_entries: &[TypeEntry]) -> Node<Msg> {
    select![
        id!("type_selector"),
        type_entries.iter().map(|type_entry| {
            let req = type_entry.load.clone();
            option![
                attrs!{
                    At::Selected => type_entry.is_selected.as_at_value(),
                    At::Value => type_entry.type_name,
                },
                simple_ev(Ev::Click, Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))),
                type_entry.type_name,
            ]
        }).collect::<Vec<_>>()
    ]
}

fn view_catalog_selector(catalog_entries: &[CatalogEntry], selected_req: Option<&ResourceRequest>) -> Option<Node<Msg>> {
    let selected_req = selected_req?;

    let catalog_entries = catalog_entries
        .iter()
        .filter(|catalog_entry| &catalog_entry.load.path.type_name == &selected_req.path.type_name);

    let catalog_groups = catalog_entries.group_by(|catalog_entry| &catalog_entry.addon_name);

    Some(select![
        id!("catalog_selector"),
        catalog_groups
            .into_iter()
            .map(|(addon_name, catalog_entries)| {
                optgroup![
                    attrs!{
                        At::Label => addon_name
                    },
                    catalog_entries.map(view_catalog_selector_item).collect::<Vec<_>>()
                ]
            }).collect::<Vec<_>>()
    ])
}

fn view_catalog_selector_item(catalog_entry: &CatalogEntry) -> Node<Msg> {
    let req = catalog_entry.load.clone();
    option![
        attrs!{
            At::Selected => catalog_entry.is_selected.as_at_value(),
            At::Value => catalog_entry.load.path.id,
        },
        simple_ev(Ev::Click, Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))),
        catalog_entry.name,
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::build(init, update, view).build_and_start();
}
