#![allow(
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::must_use_candidate
)]

mod entity;
mod helper;
mod page;
mod route;

use env_web::Env;
use futures::compat::Future01CompatExt;
use futures::FutureExt;
use helper::take;
use route::Route;
use seed::{prelude::*, *};
use std::convert::TryFrom;
use std::rc::Rc;
use stremio_core::state_types::{CatalogFiltered, Ctx, Msg as CoreMsg, Update};
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

impl Model {
    pub fn shared(&mut self) -> Option<&mut SharedModel> {
        match self {
            Self::Redirect => None,
            Self::Discover(module_model) => Some(module_model.shared()),
            Self::Detail(module_model) => Some(module_model.shared()),
            Self::Addons(module_model) => Some(module_model.shared()),
            Self::Player(shared) | Self::NotFound(shared) | Self::Board(shared) => Some(shared),
        }
    }
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
    GoTo(Route),
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
}

fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    if let GMsg::GoTo(ref route) = g_msg {
        let url = Url::try_from(route.to_href()).expect("`Url` from `Route`");
        seed::push_route(url);
    }

    let unhandled_g_msg = match model {
        Model::Discover(module_model) => {
            page::discover::sink(g_msg, module_model, &mut orders.proxy(Msg::DiscoverMsg))
        }
        Model::Addons(module_model) => {
            page::addons::sink(g_msg, module_model, &mut orders.proxy(Msg::AddonsMsg))
        }
        Model::Redirect
        | Model::Board(_)
        | Model::Detail(_)
        | Model::Player(_)
        | Model::NotFound(_) => Some(g_msg),
    };

    if let Some(unhandled_g_msg) = unhandled_g_msg {
        match unhandled_g_msg {
            GMsg::GoTo(route) => {
                orders.send_msg(Msg::RouteChanged(route));
            }
            // ------ Core  ------
            GMsg::Core(core_msg) => {
                let fx = model
                    .shared()
                    .expect("get `SharedModel` from `Model")
                    .core
                    .update(&core_msg);

                if !fx.has_changed {
                    orders.skip();
                }

                for cmd in fx.effects {
                    let cmd = cmd.compat().map(|result| {
                        result
                            .map(|core_msg| GMsg::Core(Rc::new(core_msg)))
                            .map_err(|core_msg| GMsg::CoreError(Rc::new(core_msg)))
                    });
                    orders.perform_g_cmd(cmd);
                }
            }
            GMsg::CoreError(core_error) => log!("core_error", core_error),
        };
    }
}

// ------ ------
//    Update
// ------ ------

// @TODO box large fields?
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
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
        }
        Msg::AddonsMsg(module_msg) => {
            if let Model::Addons(module_model) = model {
                page::addons::update(module_msg, module_model, &mut orders.proxy(Msg::AddonsMsg));
            }
        }
    }
}

fn change_model_by_route(route: Route, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    let shared = |model: &mut Model| SharedModel::from(take(model));
    *model = match route {
        Route::Board => Model::Board(shared(model)),
        Route::Detail {
            type_name,
            id,
            video_id,
        } => Model::Detail(page::detail::init(
            shared(model),
            type_name,
            id,
            video_id,
            &mut orders.proxy(Msg::DetailMsg),
        )),
        Route::Discover(resource_request) => Model::Discover(page::discover::init(
            shared(model),
            resource_request,
            &mut orders.proxy(Msg::DiscoverMsg),
        )),
        Route::Player => Model::Player(shared(model)),
        Route::Addons(resource_request) => Model::Addons(page::addons::init(
            shared(model),
            resource_request,
            &mut orders.proxy(Msg::AddonsMsg),
        )),
        Route::NotFound => Model::NotFound(shared(model)),
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
                Model::Discover(model) =>
                    page::discover::view(model).els().map_msg(Msg::DiscoverMsg),
                Model::Detail(_) => page::detail::view().els(),
                Model::Player(_) => page::player::view().els(),
                Model::Addons(model) => page::addons::view(model).els().map_msg(Msg::AddonsMsg),
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
    App::builder(update, view)
        .routes(routes)
        .sink(sink)
        .build_and_start();
}
