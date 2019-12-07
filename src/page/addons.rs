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
                // @TODO
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
                // @TODO
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
