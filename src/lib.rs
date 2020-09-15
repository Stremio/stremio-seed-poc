#![allow(
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

mod entity;
mod page;

use env_web::Env;
use futures::compat::Future01CompatExt;
use seed::{prelude::*, *};
use std::rc::Rc;
use std::str::FromStr;
use stremio_core::state_types::{CatalogFiltered, Ctx, Msg as CoreMsg, Update};
use stremio_core::types::addons::{
    DescriptorPreview, ParseResourceErr, ResourceRef, ResourceRequest,
};
use stremio_core::types::MetaPreview;
use stremio_derive::Model;

// ---- url parts ----

const BOARD: &str = "board";
const DISCOVER: &str = "discover";
const DETAIL: &str = "detail";
const PLAYER: &str = "player";
const ADDONS: &str = "addons";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let root_base_url = url.to_hash_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(|UpdateCoreModel(core_msg)| Msg::CoreMsg(core_msg))
        .subscribe(Msg::CoreMsg)
        .notify(subs::UrlChanged(url));

    Model {
        context: Context {
            core_model: CoreModel::default(),
            root_base_url,
        },
        page_id: None,
        // ---- page models ----
        detail_model: None,
        discover_model: None,
        addons_model: None,
    }
}

#[derive(Clone)]
struct UpdateCoreModel(Rc<CoreMsg>);

// ------ ------
//     Model
// ------ ------

// @TODO box large fields?
#[allow(clippy::large_enum_variant)]
pub struct Model {
    context: Context,
    page_id: Option<PageId>,
    // ---- page models ----
    detail_model: Option<page::detail::Model>,
    discover_model: Option<page::discover::Model>,
    addons_model: Option<page::addons::Model>,
}

// ------ Context ------

pub struct Context {
    core_model: CoreModel,
    root_base_url: Url,
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageId {
    Board,
    Detail,
    Discover,
    Player,
    Addons,
    NotFound,
}

// ------ CoreModel  ------

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
    addon_catalog: CatalogFiltered<DescriptorPreview>,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn board(self) -> Url {
        self.base_url().add_hash_path_part(BOARD)
    }
    pub fn discover_urls(self) -> page::discover::Urls<'a> {
        page::discover::Urls::new(self.base_url().add_hash_path_part(DISCOVER))
    }
    pub fn detail_urls(self) -> page::detail::Urls<'a> {
        page::detail::Urls::new(self.base_url().add_hash_path_part(DETAIL))
    }
    pub fn player(self) -> Url {
        self.base_url().add_hash_path_part(PLAYER)
    }
    pub fn addons_urls(self) -> page::addons::Urls<'a> {
        page::addons::Urls::new(self.base_url().add_hash_path_part(ADDONS))
    }
}

// ------ ------
//  Url helpers
// ------ ------

fn resource_request_to_path_parts(req: &ResourceRequest) -> Vec<String> {
    vec![req.base.to_owned(), req.path.to_string()]
}

#[derive(Debug)]
enum ParseResourceRequestError {
    Resource(ParseResourceErr),
}

fn resource_request_try_from_url_parts(
    base: &str,
    path: &str,
) -> Result<ResourceRequest, ParseResourceRequestError> {
    Ok(ResourceRequest {
        base: base.to_owned(),
        path: ResourceRef::from_str(&path).map_err(ParseResourceRequestError::Resource)?,
    })
}

// ------ ------
//    Update
// ------ ------

// @TODO box large fields?
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
enum Msg {
    UrlChanged(subs::UrlChanged),
    CoreMsg(Rc<CoreMsg>),
    CoreCmdFinished(Rc<CoreMsg>),
    DiscoverMsg(page::discover::Msg),
    DetailMsg(page::detail::Msg),
    AddonsMsg(page::addons::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page_id = Some(init_page(url, model, orders));
        }
        Msg::CoreMsg(core_msg) => {
            let fx = model.context.core_model.update(&core_msg);

            if !fx.has_changed {
                orders.skip();
            }

            for cmd in fx.effects {
                orders.perform_cmd(async move {
                    match cmd.compat().await {
                        Ok(core_msg) | Err(core_msg) => Msg::CoreCmdFinished(Rc::new(core_msg)),
                    }
                });
            }
        }
        Msg::CoreCmdFinished(core_msg) => {
            orders.notify(core_msg);
        }
        Msg::DiscoverMsg(page_msg) => {
            if let Some(page_model) = &mut model.discover_model {
                page::discover::update(
                    page_msg,
                    page_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::DiscoverMsg),
                );
            }
        }
        Msg::DetailMsg(page_msg) => {
            if let Some(page_model) = &mut model.detail_model {
                page::detail::update(page_msg, page_model, &mut orders.proxy(Msg::DetailMsg));
            }
        }
        Msg::AddonsMsg(page_msg) => {
            if let Some(page_model) = &mut model.addons_model {
                page::addons::update(
                    page_msg,
                    page_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::AddonsMsg),
                );
            }
        }
    }
}

fn init_page(mut url: Url, model: &mut Model, orders: &mut impl Orders<Msg>) -> PageId {
    match url.next_hash_path_part() {
        None | Some(BOARD) => Some(PageId::Board),
        Some(DISCOVER) => {
            page::discover::init(
                url,
                &mut model.discover_model,
                &mut orders.proxy(Msg::DiscoverMsg),
            )
        }
        Some(DETAIL) => {
            page::detail::init(
                url,
                &mut model.detail_model,
                &mut orders.proxy(Msg::DetailMsg),
            )
        }
        Some(PLAYER) => Some(PageId::Player),
        Some(ADDONS) => {
            page::addons::init(
                url,
                &mut model.addons_model,
                &mut orders.proxy(Msg::AddonsMsg),
            )
        }
        _ => None,
    }.unwrap_or(PageId::NotFound)
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["router", "routes-container"],
        div![
            C!["route-container",],
            model.page_id.map(|page_id| {
                match page_id {
                    PageId::Board => page::board::view(&model.context.root_base_url).into_nodes(),
                    PageId::Detail => page::detail::view().into_nodes(),
                    PageId::Discover => {
                        if let Some(page_model) = &model.discover_model {
                            page::discover::view(page_model, &model.context)
                                .map_msg(Msg::DiscoverMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
                    PageId::Player => page::player::view().into_nodes(),
                    PageId::Addons => {
                        if let Some(page_model) = &model.addons_model {
                            page::addons::view(page_model, &model.context).map_msg(Msg::AddonsMsg)
                        } else {
                            vec![]
                        }
                    }
                    PageId::NotFound => page::not_found::view().into_nodes(),
                }
            })
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
