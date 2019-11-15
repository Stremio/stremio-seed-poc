use seed::{prelude::*, *};
use std::rc::Rc;
use stremio_core::state_types::{Action, ActionLoad, Loadable, TypeEntry, CatalogEntry, CatalogError, Msg as CoreMsg, Update};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::ResourceRequest;
use crate::{default_resource_request, entity::multi_select, SharedModel, route::Route};
use futures::future::Future;
use std::convert::TryFrom;

mod type_selector;
mod catalog_selector;
mod extra_prop_selector;

type MetaPreviewId = String;
// @TODO add into stremio-core?
type ExtraPropOption = String;

// ------ ------
//     Model
// ------ ------

pub struct Model {
    shared: SharedModel,
    selected_meta_preview_id: Option<MetaPreviewId>,
    type_selector_model: type_selector::Model,
    catalog_selector_model: catalog_selector::Model,
    extra_prop_selector_model: extra_prop_selector::Model,
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
        // @TODO try to remove `Clone` requiremnt from Seed or add it into stremi-core? Implement intos, from etc.?
        Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(resource_request)))))
    );

    Model {
        type_selector_model: type_selector::init(),
        catalog_selector_model: catalog_selector::init(),
        extra_prop_selector_model: extra_prop_selector::init(),
        selected_meta_preview_id: None,
        shared,
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    MetaPreviewClicked(MetaPreviewId),
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    TypeSelectorMsg(type_selector::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<TypeEntry>>),
    CatalogSelectorMsg(catalog_selector::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    ExtraPropSelectorMsg(extra_prop_selector::Msg),
    ExtraPropSelectorChanged(Vec<multi_select::Group<ExtraPropOption>>),
}

fn push_resource_request(req: ResourceRequest, orders: &mut impl Orders<Msg>) {
    let route = Route::Discover(req.clone());
    let url = Url::try_from(route.to_href()).expect("`Url` from `Route::Discover`");
    seed::push_route(url);

    orders.send_msg(
        Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req)))))
    );
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let catalog = &model.shared.core.catalog;

    match msg {
        Msg::MetaPreviewClicked(meta_preview_id) => {
            model.selected_meta_preview_id = Some(meta_preview_id);
        },

        // ------ Core  ------

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

        // ------ TypeSelector  ------

        Msg::TypeSelectorMsg(msg) => {
            let msg_to_parent = type_selector::update(
                msg,
                &mut model.type_selector_model,
                &mut orders.proxy(Msg::TypeSelectorMsg),
                type_selector::groups(&catalog.types),
                Msg::TypeSelectorChanged
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::TypeSelectorChanged(groups_with_selected_items) => {
            let req = type_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        },

        // ------ CatalogSelector  ------

        Msg::CatalogSelectorMsg(msg) => {
            let msg_to_parent = catalog_selector::update(
                msg,
                &mut model.catalog_selector_model,
                &mut orders.proxy(Msg::CatalogSelectorMsg),
                catalog_selector::groups(&catalog.catalogs, &catalog.selected),
                Msg::CatalogSelectorChanged
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::CatalogSelectorChanged(groups_with_selected_items) => {
            let req = catalog_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        },

        // ------ ExtraPropSelector  ------

        Msg::ExtraPropSelectorMsg(msg) => {
            let msg_to_parent = extra_prop_selector::update(
                msg,
                &mut model.extra_prop_selector_model,
                &mut orders.proxy(Msg::ExtraPropSelectorMsg),
                extra_prop_selector::groups(&catalog.selectable_extra, &catalog.selected),
                Msg::ExtraPropSelectorChanged
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        },
        Msg::ExtraPropSelectorChanged(groups_with_selected_items) => {
            if let Some(req) = extra_prop_selector::resource_request(groups_with_selected_items, &catalog.selected) {
                push_resource_request(req, orders)
            }
        },
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let catalog = &model.shared.core.catalog;

    div![
        id!("discover"),
        div![
            // type selector
            type_selector::view(
                &model.type_selector_model,
                &type_selector::groups(&catalog.types)
            ).map_message(Msg::TypeSelectorMsg),

            // catalog selector
            catalog_selector::view(
                &model.catalog_selector_model,
                &catalog_selector::groups(&catalog.catalogs, &catalog.selected))
            .map_message(Msg::CatalogSelectorMsg),

            // extra prop selector
            extra_prop_selector::view(
                &model.extra_prop_selector_model,
                &extra_prop_selector::groups(&catalog.selectable_extra, &catalog.selected))
            .map_message(Msg::ExtraPropSelectorMsg),

            // reset button
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
    //let poster_shape = meta_preview.poster_shape.to_str();

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
        div![
            class![
                "thumb"
            ],
            style!{
                St::BackgroundImage => format!("url({})", poster)
            },
        ],
        a![
            class![
                "icon",
                "icon-ic_play",
                "button"
            ],
            attrs!{
                At::Href => if is_selected { AtValue::Some(Route::Player.to_href()) } else { AtValue::Ignored }
            }
        ]
    ]
}
