#![allow(clippy::needless_pass_by_value)]

mod entity;
mod helper;
mod page;
mod route;

use env_web::Env;
use helper::take;
use route::Route;
use seed::{prelude::*, App};
use stremio_core::state_types::{CatalogFiltered, Ctx};
use stremio_core::types::addons::{ResourceRef, ResourceRequest};
use stremio_core::types::MetaPreview;
use stremio_derive::Model;

fn default_resource_request() -> ResourceRequest {
    ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("catalog", "movie", "top"),
    }
}

// ------ ------
//     Model
// ------ ------

pub enum Model {
    Redirect,
    Board(SharedModel),
    Detail(SharedModel),
    Discover(page::discover::Model),
    NotFound(SharedModel),
    Player(SharedModel),
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
            Model::Board(shared_model) => shared_model,
            Model::Detail(shared_model) => shared_model,
            Model::Discover(module_model) => module_model.into(),
            Model::NotFound(shared_model) => shared_model,
            Model::Player(shared_model) => shared_model,
        }
    }
}

// ------ CoreModel  ------

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
}

// ------ ------
//    Routes
// ------ ------

fn routes(url: Url) -> Option<Msg> {
    Some(Msg::RouteChanged(url.into()))
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum Msg {
    RouteChanged(Route),
    DiscoverMsg(page::discover::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
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
    }
}

fn change_model_by_route<'a>(route: Route, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let shared_model = SharedModel::from(take(model));
    *model = match route {
        Route::Board => Model::Board(shared_model),
        Route::Detail => Model::Detail(shared_model),
        Route::Discover(resource_request) => Model::Discover(page::discover::init(
            shared_model,
            resource_request,
            &mut orders.proxy(Msg::DiscoverMsg),
        )),
        Route::NotFound => Model::NotFound(shared_model),
        Route::Player => Model::Player(shared_model),
    };
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    match &model {
        Model::Redirect => page::blank::view(),
        Model::Board(_) => page::board::view(),
        Model::Discover(model) => page::discover::view(&model).map_message(Msg::DiscoverMsg),
        Model::Detail(_) => page::detail::view(),
        Model::Player(_) => page::player::view(),
        Model::NotFound(_) => page::not_found::view(),
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::build(|_, _| Init::default(), update, view)
        .routes(routes)
        .build_and_start();
}
