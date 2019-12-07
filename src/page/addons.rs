use seed::{prelude::*, *};
use crate::{entity::multi_select, route::Route, SharedModel};
use std::rc::Rc;
use std::convert::TryFrom;
use futures::future::Future;
use stremio_core::state_types::{
    Action, ActionLoad, CatalogEntry, CatalogError, Loadable, Msg as CoreMsg, TypeEntry, Update,
};
use stremio_core::types::{addons::{ResourceRequest, ResourceRef}, PosterShape};

mod catalog_selector;
mod type_selector;

const DEFAULT_CATALOG: &str = "thirdparty";
const DEFAULT_TYPE: &str = "movie";

pub fn default_resource_request() -> ResourceRequest {
    ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("addon_catalog", DEFAULT_TYPE, DEFAULT_CATALOG),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    shared: SharedModel,
    catalog_selector_model: catalog_selector::Model,
    type_selector_model: type_selector::Model,
    search_query: String,
}

impl From<Model> for SharedModel {
    fn from(model: Model) -> Self {
        model.shared
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(
    shared: SharedModel,
    resource_request: Option<ResourceRequest>,
    orders: &mut impl Orders<Msg>,
) -> Model {
    orders.send_msg(
        // @TODO try to remove `Clone` requirement from Seed or add it into stremi-core? Implement intos, from etc.?
        // @TODO select the first preview on Load
        Msg::Core(Rc::new(CoreMsg::Action(Action::Load(
            ActionLoad::CatalogFiltered(resource_request.unwrap_or_else(|| default_resource_request())),
        )))),
    );

    Model {
        shared,
        catalog_selector_model: catalog_selector::init(),
        type_selector_model: type_selector::init(),
        search_query: String::new(),
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::pub_enum_variant_names)]
#[derive(Clone)]
pub enum Msg {
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    CatalogSelectorMsg(catalog_selector::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    TypeSelectorMsg(type_selector::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<TypeEntry>>),
    SearchQueryChanged(String),
    AddAddonButtonClicked,
}

fn push_resource_request(req: ResourceRequest, orders: &mut impl Orders<Msg>) {
    let route = Route::Addons(Some(req.clone()));
    let url = Url::try_from(route.to_href()).expect("`Url` from `Route::Addons`");
    seed::push_route(url);

    orders.send_msg(Msg::Core(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::CatalogFiltered(req),
    )))));
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let catalog = &model.shared.core.addon_catalog;

    match msg {
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
        }
        Msg::CoreError(core_error) => log!("core_error", core_error),

        // ------ CatalogSelector  ------
        Msg::CatalogSelectorMsg(msg) => {
            let msg_to_parent = catalog_selector::update(
                msg,
                &mut model.catalog_selector_model,
                &mut orders.proxy(Msg::CatalogSelectorMsg),
                catalog_selector::groups(&catalog.catalogs, &catalog.selected),
                Msg::CatalogSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::CatalogSelectorChanged(groups_with_selected_items) => {
            let req = catalog_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        }

        // ------ TypeSelector  ------
        Msg::TypeSelectorMsg(msg) => {
            let msg_to_parent = type_selector::update(
                msg,
                &mut model.type_selector_model,
                &mut orders.proxy(Msg::TypeSelectorMsg),
                type_selector::groups(&catalog.types),
                Msg::TypeSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::TypeSelectorChanged(groups_with_selected_items) => {
            let req = type_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        }

        Msg::SearchQueryChanged(search_query) => model.search_query = search_query,
        Msg::AddAddonButtonClicked => log!("add addon button clicked"),
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let catalog = &model.shared.core.addon_catalog;

    div![
        class!["addons-container"],
        div![
            class!["addons-content"],
            div![
                class!["top-bar-container"],
                // add addon button
                view_add_addon_button(),
                // catalog selector
                catalog_selector::view(
                    &model.catalog_selector_model,
                    &catalog_selector::groups(&catalog.catalogs, &catalog.selected)
                )
                .map_message(Msg::CatalogSelectorMsg),
                // type selector
                type_selector::view(
                    &model.type_selector_model,
                    &type_selector::groups(&catalog.types)
                )
                .map_message(Msg::TypeSelectorMsg),
                // search input
                view_search_input(&model.search_query),
            ],
            div![
                class!["addons-list-container"],
//                view_content(
//                    &model.shared.core.catalog.content,
//                    model.selected_meta_preview_id.as_ref()
//                ),
            ]
        ],
    ]
}

fn view_add_addon_button() -> Node<Msg> {
    div![
        class![
            "add-button-container", "button-container",
        ],
        attrs!{
            At::TabIndex => 0,
        },
        simple_ev(Ev::Click, Msg::AddAddonButtonClicked),
        svg![
            class!["icon",],
            attrs! {
                At::ViewBox => "0 0 1024 1024",
                "icon" => "ic_plus",
            },
            path![attrs! {
                At::D => "M576.151 576.151h383.699c35.429 0 64.151-28.721 64.151-64.151s-28.721-64.151-64.151-64.151v-0h-383.699v-383.699c0-35.429-28.721-64.151-64.151-64.151s-64.151 28.721-64.151 64.151h-0v383.699h-383.699c-35.429 0-64.151 28.721-64.151 64.151s28.721 64.151 64.151 64.151v0h383.699v383.699c0 35.429 28.721 64.151 64.151 64.151s64.151-28.721 64.151-64.151v0z"
            }]
        ],
        div![
            class![
                "add-button-label"
            ],
            "Add addon"
        ]
    ]
}

fn view_search_input(search_query: &str) -> Node<Msg> {
    div![
        class![
            "search-bar-container",
        ],
        svg![
            class!["icon",],
            attrs! {
                At::ViewBox => "0 0 1025 1024",
                "icon" => "ic_search",
            },
            path![attrs! {
                At::D => "M1001.713 879.736c-48.791-50.899-162.334-163.84-214.438-216.546 43.772-66.969 69.909-148.918 70.174-236.956l0-0.070c-1.877-235.432-193.166-425.561-428.862-425.561-236.861 0-428.875 192.014-428.875 428.875 0 236.539 191.492 428.353 427.909 428.874l0.050 0c1.551 0.021 3.382 0.033 5.216 0.033 85.536 0 165.055-25.764 231.219-69.956l-1.518 0.954 201.487 204.499c16.379 18.259 39.94 29.789 66.201 30.117l0.058 0.001c2.034 0.171 4.401 0.269 6.791 0.269 35.32 0 65.657-21.333 78.83-51.816l0.214-0.556c5.589-10.528 8.87-23.018 8.87-36.275 0-21.857-8.921-41.631-23.32-55.878l-0.007-0.007zM429.478 730.654c-0.004 0-0.008 0-0.012 0-166.335 0-301.176-134.841-301.176-301.176 0-0.953 0.004-1.905 0.013-2.856l-0.001 0.146c0.599-165.882 135.211-300.124 301.176-300.124 166.336 0 301.178 134.842 301.178 301.178 0 0.371-0.001 0.741-0.002 1.111l0-0.057c0 0.179 0.001 0.391 0.001 0.603 0 166.335-134.841 301.176-301.176 301.176-0.106 0-0.212-0-0.318-0l0.016 0z"
            }]
        ],
        input![
            class![
                "search-input", "text-input"
            ],
            attrs!{
                At::TabIndex => 0,
                At::Placeholder => "Search addons...",
                At::Value => search_query,
            },
            input_ev(Ev::Input, Msg::SearchQueryChanged),
        ]
    ]
}
