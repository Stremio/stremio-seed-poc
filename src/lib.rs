#![allow(clippy::needless_pass_by_value, clippy::non_ascii_literal)]

mod entity;
mod helper;
mod page;
mod route;

use env_web::Env;
use helper::take;
use route::Route;
use seed::{prelude::*, *};
use stremio_core::state_types::{CatalogFiltered, Ctx};
use stremio_core::types::addons::DescriptorPreview;
use stremio_core::types::MetaPreview;
use stremio_derive::Model;

// ------ ------
//     Model
// ------ ------

// @TODO box large fields?
#[allow(clippy::large_enum_variant)]
pub enum Model {
    Redirect,
    Board(SharedModel),
    Detail(page::detail::Model),
    Discover(page::discover::Model),
    Player(SharedModel),
    Addons(page::addons::Model),
    NotFound(SharedModel),
}

impl Default for Model {
    fn default() -> Self {
        Self::Redirect
    }
}

// ------ SharedModel  ------

#[derive(Default)]
pub struct SharedModel {
    core: CoreModel,
}

impl From<Model> for SharedModel {
    fn from(model: Model) -> Self {
        match model {
            Model::Redirect => Self::default(),
            Model::Discover(module_model) => module_model.into(),
            Model::Detail(module_model) => module_model.into(),
            Model::Addons(module_model) => module_model.into(),
            Model::Board(shared_model)
            | Model::Player(shared_model)
            | Model::NotFound(shared_model) => shared_model,
        }
    }
}

// ------ CoreModel  ------

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
    addon_catalog: CatalogFiltered<DescriptorPreview>,
}

// ------ ------
//    Routes
// ------ ------

fn routes(url: Url) -> Option<Msg> {
    Some(Msg::RouteChanged(url.into()))
}

// ------ ------
//     Sink
// ------ ------

pub enum GMsg {
    RoutePushed(Route),
}

fn sink(g_msg: GMsg, _: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::RoutePushed(route) => {
            orders.send_msg(Msg::RouteChanged(route))
        }
    };
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
enum Msg {
    RouteChanged(Route),
    DiscoverMsg(page::discover::Msg),
    DetailMsg(page::detail::Msg),
    AddonsMsg(page::addons::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        }
        Msg::DiscoverMsg(module_msg) => {
            if let Model::Discover(module_model) = model {
                page::discover::update(
                    module_msg,
                    module_model,
                    &mut orders.proxy(Msg::DiscoverMsg),
                );
            }
        }
        Msg::DetailMsg(module_msg) => {
            if let Model::Detail(module_model) = model {
                page::detail::update(module_msg, module_model, &mut orders.proxy(Msg::DetailMsg));
            }
        },
        Msg::AddonsMsg(module_msg) => {
            if let Model::Addons(module_model) = model {
                page::addons::update(module_msg, module_model, &mut orders.proxy(Msg::AddonsMsg));
            }
        }
    }
}

fn change_model_by_route(route: Route, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    let shared_model = SharedModel::from(take(model));
    *model = match route {
        Route::Board => Model::Board(shared_model),
        Route::Detail { type_name, id, video_id } => {
            Model::Detail(page::detail::init(shared_model, type_name, id, video_id, &mut orders.proxy(Msg::DetailMsg)))
        },
        Route::Discover(resource_request) => Model::Discover(page::discover::init(
            shared_model,
            resource_request,
            &mut orders.proxy(Msg::DiscoverMsg),
        )),
        Route::Player => Model::Player(shared_model),
        Route::Addons(resource_request) => Model::Addons(page::addons::init(
            shared_model,
            resource_request,
            &mut orders.proxy(Msg::AddonsMsg),
        )),
        Route::NotFound => Model::NotFound(shared_model),
    };
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
        class!["router", "routes-container"],
        div![
            class!["route-container",],
            match &model {
                Model::Redirect => page::blank::view().els(),
                Model::Board(_) => page::board::view().els(),
                Model::Discover(model) => page::discover::view(model)
                    .els()
                    .map_message(Msg::DiscoverMsg),
                Model::Detail(_) => page::detail::view().els(),
                Model::Player(_) => page::player::view().els(),
                Model::Addons(model) => page::addons::view(model).els().map_message(Msg::AddonsMsg),
                Model::NotFound(_) => page::not_found::view().els(),
            }
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).routes(routes).sink(sink).build_and_start();
}
