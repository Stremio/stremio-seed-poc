#![allow(
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::must_use_candidate
)]

mod entity;
mod page;

use env_web::Env;
use futures::compat::Future01CompatExt;
use seed::{prelude::*, *};
use std::rc::Rc;
use std::ops::Deref;
use std::str::FromStr;
use stremio_core::state_types::{CatalogFiltered, Ctx, Msg as CoreMsg, Update};
use stremio_core::types::MetaPreview;
use stremio_derive::Model;
use stremio_core::types::addons::{DescriptorPreview, ParseResourceErr, ResourceRef, ResourceRequest};

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
enum PageId {
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
    pub fn discover(self, res_req: Option<ResourceRequest>) -> Url {
        let url = self
            .base_url()
            .add_hash_path_part(DISCOVER);

        let url = resource_request_to_path_parts(&res_req)
            .into_iter()
            .fold(url, |url, path_part| {
                url.add_hash_path_part(path_part)
            });
        url
    }
    pub fn detail(self, type_name: String, id: String, video_id: Option<String>,) -> Url {
        self
            .base_url()
            .add_hash_path_part(DETAIL)
            .add_hash_path_part(type_name)
            .add_hash_path_part(id)
            .add_hash_path_part(video_id.unwrap_or_default())
    }
    pub fn player(self) -> Url {
        self.base_url().add_hash_path_part(PLAYER)
    }
    pub fn addons(self, res_req: Option<ResourceRequest>) -> Url {
        let url = self
            .base_url()
            .add_hash_path_part(ADDONS);

        let url = resource_request_to_path_parts(&res_req)
            .into_iter()
            .fold(url, |url, path_part| {
                url.add_hash_path_part(path_part)
            });
        url
    }
}

fn resource_request_to_path_parts(req: &Option<ResourceRequest>) -> Vec<String> {
    let req = if let Some(req) = req {
        req
    } else {
        return Vec::new();
    };

    // @TODO do we have to encode it?
    let encoded_base = String::from(js_sys::encode_uri_component(&req.base));
    let encoded_path = String::from(js_sys::encode_uri_component(&req.path.to_string()));
    vec![encoded_base, encoded_path]
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
        },
        Msg::CoreMsg(core_msg) => {
            let fx = model
                .context
                .core_model
                .update(&core_msg);

            if !fx.has_changed {
                orders.skip();
            }

            for cmd in fx.effects {
                // @TODO ?
                orders.perform_cmd(async move {
                    match cmd.compat().await {
                        Ok(core_msg) | Err(core_msg) => Msg::CoreCmdFinished(Rc::new(core_msg))
                    }
                });

                // let cmd = cmd.compat().map(|result| {
                //     result
                //         .map(|core_msg| GMsg::Core(Rc::new(core_msg)))
                //         .map_err(|core_msg| GMsg::CoreError(Rc::new(core_msg)))
                // });
                // orders.perform_g_cmd(cmd);

                // GMsg::CoreError(core_error) => log!("core_error", core_error),
            }
        },
        Msg::CoreCmdFinished(core_msg) => {
            orders.notify(core_msg);
        },
        Msg::DiscoverMsg(page_msg) => {
            if let Some(page_model) = &mut model.discover_model{
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
                    &mut orders.proxy(Msg::AddonsMsg)
                );
            }
        }
    }
}

fn init_page(mut url: Url, model: &mut Model, orders: &mut impl Orders<Msg>) -> PageId {
    match url.remaining_hash_path_parts().as_slice() {
        [] | [BOARD] => PageId::Board,
        [DISCOVER] => {
            page::discover::init(&mut model.discover_model, None, &mut orders.proxy(Msg::DiscoverMsg));
            PageId::Discover
        }
        [DISCOVER, encoded_base, encoded_path] => {
            let resource_request = match resource_request_try_from_url_parts(encoded_base, encoded_path) {
                Ok(req) => req,
                Err(error) => {
                    error!(error);
                    return PageId::NotFound;
                }
            };
            page::discover::init(&mut model.discover_model, Some(resource_request), &mut orders.proxy(Msg::DiscoverMsg));
            PageId::Discover
        }
        [DETAIL, type_name, id, rest @ ..] => {
            let video_id = rest.first().map(Deref::deref);
            page::detail::init(
                model.detail_model.as_mut(), 
                type_name, 
                id, 
                video_id, 
                &mut orders.proxy(Msg::DetailMsg)
            );
            PageId::Detail
        },
        [PLAYER] => PageId::Player,
        [ADDONS] => {
            page::addons::init(&mut model.addons_model, None, &mut orders.proxy(Msg::AddonsMsg));
            PageId::Addons
        }
        [ADDONS, encoded_base, encoded_path] => {
            let resource_request = match resource_request_try_from_url_parts(encoded_base, encoded_path) {
                Ok(req) => req,
                Err(error) => {
                    error!(error);
                    return PageId::NotFound;
                }
            };
            page::addons::init(&mut model.addons_model, Some(resource_request), &mut orders.proxy(Msg::AddonsMsg));
            PageId::Addons
        }
        _ => PageId::NotFound,
    }
}

#[derive(Debug)]
enum ParseResourceRequestError {
    UriDecode(String),
    Resource(ParseResourceErr),
}

fn resource_request_try_from_url_parts(
    uri_encoded_base: &str,
    uri_encoded_path: &str,
) -> Result<ResourceRequest, ParseResourceRequestError> {
    // @TODO do we have to decode it?

    let base: String = {
        js_sys::decode_uri_component(uri_encoded_base)
            .map_err(|_| ParseResourceRequestError::UriDecode(uri_encoded_base.to_owned()))?
            .into()
    };

    let path: String = {
        js_sys::decode_uri_component(uri_encoded_path)
            .map_err(|_| ParseResourceRequestError::UriDecode(uri_encoded_path.to_owned()))?
            .into()
    };

    Ok(ResourceRequest {
        base,
        path: ResourceRef::from_str(&path).map_err(ParseResourceRequestError::Resource)?,
    })
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
                            page::discover::view(page_model, &model.context).map_msg(Msg::DiscoverMsg).into_nodes()
                        } else {
                            vec![]
                        }
                    },
                    PageId::Player => page::player::view().into_nodes(),
                    PageId::Addons => {
                        if let Some(page_model) = &model.addons_model {
                            page::addons::view(page_model, &model.context).map_msg(Msg::AddonsMsg)
                        } else {
                            vec![]
                        }
                    },
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
