#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate seed;
use env_web::Env;
use seed::{prelude::*, App, *};
use stremio_core::state_types::{Action, ActionLoad, CatalogFiltered, Ctx, Loadable, Msg, Update, TypeEntry, CatalogEntry, CatalogError};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::{ResourceRequest, ResourceRef, ManifestExtraProp, ExtraProp};
use stremio_derive::Model;
use itertools::Itertools;

// ------ ------
//     Model
// ------ ------

#[derive(Model, Default)]
struct Model {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
}

// ------ ------
//     Init
// ------ ------

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    orders.send_msg(default_load());
    Init::default()
}

// ------ ------
//    Update
// ------ ------

fn default_load() -> Msg {
    let req = ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("catalog", "movie", "top"),
    };
    Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let fx = model.update(&msg);
    if !fx.has_changed {
        orders.skip();
    }
    for cmd in fx.effects {
        orders.perform_cmd(cmd);
    }
}

// ------ ------
//     View
// ------ ------

// @TODO reset button?
fn view(model: &Model) -> Node<Msg> {
//    log!("TYPES:", model.catalog.types);
//    log!("CATALOGS:", model.catalog.catalogs);
//    log!("Extra:", model.catalog.selectable_extra);
    log!("CONTENT", model.catalog.content);

    div![
        id!("discover"),
        div![
            view_type_selector(&model.catalog.types),
            view_catalog_selector(&model.catalog.catalogs, model.catalog.selected.as_ref()).unwrap_or_else(|| empty![]),
            view_extra_prop_selector(&model.catalog.selectable_extra, model.catalog.selected.as_ref()).unwrap_or_else(|| empty![]),
        ],
        div![
            id!("discover_holder"),
            style!{
                St::Top => px(40),
            },
            class![
                "holder",
            ],
            view_content(&model.catalog.content),
        ]
    ]
}

fn view_content(content: &Loadable<Vec<MetaPreview>, CatalogError>) -> Node<Msg> {
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
                meta_previews.iter().map(view_meta_preview).collect::<Vec<_>>()
            ]
        }
    }
}

fn view_meta_preview(meta_preview: &MetaPreview) -> Node<Msg> {
    let default_poster = "https://www.stremio.com/images/add-on-money.png".to_owned();
    let poster = meta_preview.poster.as_ref().unwrap_or(&default_poster);
    let poster_shape = meta_preview.poster_shape.to_str();

    li![
        class![
            "selected" => true,
            "item"
        ],
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

//    div![
//        attrs! {
//            At::Class => format!("meta-item meta-item-container poster-shape-{}", poster_shape);
//            At::Title => meta_preview.name
//        },
//        div![
//            class!["poster-image-container"],
//            div![
//                class!["poster-image-layer"],
//                div![
//                    class!["poster-image"],
//                    style! { "background-image" => format!("url({})", poster) },
////                    raw_ev(Ev::Click, |_| default_load()) //raw_ev(Ev::Click, |_| Msg::Action(Action::UserOp(ActionUser::Login{ email, password })))
//                ]
//            ]
//        ],
//        div![
//            class!["title-bar-container"],
//            div![class!["title"], meta_preview.name]
//        ],
//    ]
}

//<li ng-repeat="item in resp.metas track by item._id" ng-click="selectItem(item)" ng-class="{ inLib: libraryItem.byId[item.id], selected: item._id == info._id }" spatial-nav-enter="open({ metaItem: item })" spatial-nav-focus="$parent.info = item" tabindex="-1" class="item ng-scope">
//    <div class="name ng-binding">
//        Cocomelon - Nursery Rhymes
//    </div>
//
//    <a stremio-image="::{ url: item.poster }" class="thumb" data-image="https://yt3.ggpht.com/a/AGF-l79wZ6qBUvS5bcIe_XVWu7cUdHEEZRUnK18Pcg=s800-c-k-c0xffffffff-no-rj-mo" style="background-image: url(&quot;https://yt3.ggpht.com/a/AGF-l79wZ6qBUvS5bcIe_XVWu7cUdHEEZRUnK18Pcg=s800-c-k-c0xffffffff-no-rj-mo&quot;);">
//    </a>
//
//    <div class="icon icon-ic_play button">
//    </div>
//
//    <!-- ngIf: ::item.featured --><!-- ngIf: ::!item.featured && item.isInTheaterOnly() -->
//</li>

fn view_extra_prop_selector(extra_props: &[ManifestExtraProp], selected_req: Option<&ResourceRequest>) -> Option<Node<Msg>> {
    let selected_req = selected_req?;
    let selected_extra_props = &selected_req.path.extra;

    log!("extra_props", extra_props);
    log!("selected_req", selected_req);

    let mut select_is_empty = true;
    let select =
        select![
            attrs!{
                At::Value => "sef",
            },
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
                        option_nodes
                    ]
                }).collect::<Vec<_>>()
        ];

    if select_is_empty {
        return None
    }
    Some(select)
}

// @TODO: respect limit, handle required/optional, show custom value, don't hide on click? ...
fn view_extra_prop_selector_items(extra_prop: &ManifestExtraProp, selected_req: &ResourceRequest) -> Vec<Node<Msg>> {
    let options = match &extra_prop.options {
        Some(options) => options,
        None => return Vec::new()
    };

    options.iter().map(|option| {
        let option_is_selected = selected_req.path.extra
            .iter()
            .any(|(selected_name, selected_option)| {
                selected_name == &extra_prop.name && selected_option == option
            });

        let mut req = selected_req.clone();
        if option_is_selected {
            req.path.extra.retain(|(selected_name, selected_option)| {
                selected_name != &extra_prop.name || selected_option != option
            });
        } else {
            req.path.extra.push((extra_prop.name.clone(), option.clone()));
        }

        option![
            attrs!{
                At::Selected => option_is_selected.as_at_value(),
                At::Value => option,
            },
            raw_ev(Ev::Click, |_| Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))),
            option,
            if option_is_selected { " ✓" } else { "" },
        ]
    }).collect()
}

fn view_type_selector(type_entries: &[TypeEntry]) -> Node<Msg> {
    select![
        type_entries.iter().map(|type_entry| {
            let req = type_entry.load.clone();
            option![
                attrs!{
                    At::Selected => type_entry.is_selected.as_at_value(),
                    At::Value => type_entry.type_name,
                },
                raw_ev(Ev::Click, |_| Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))),
                type_entry.type_name
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
            At::Value => catalog_entry.name,
        },
        raw_ev(Ev::Click, |_| Msg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))),
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
