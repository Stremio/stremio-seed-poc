#![allow(clippy::needless_pass_by_value)]

mod page;
mod entity;

use env_web::Env;
use seed::{prelude::*, App, *};
use stremio_core::state_types::{Action, ActionLoad, CatalogFiltered, Ctx, Msg as CoreMsg, Update};
use stremio_core::types::MetaPreview;
use stremio_core::types::addons::{ResourceRequest, ResourceRef, ParseResourceErr};
use stremio_derive::Model;
use itertools::Itertools;
use futures::future::Future;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use std::borrow::Cow;
use std::str::FromStr;
use crate::Page::NotFound;

// ------ ------
//     Model
// ------ ------

pub type MetaPreviewId = String;

pub struct Model {
    core: CoreModel,
    selected_meta_preview_id: Option<MetaPreviewId>,
    page: Page,
}

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<Env>,
    catalog: CatalogFiltered<MetaPreview>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Page {
    Board,
    Discover(ResourceRequest),
    Detail,
    Player,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> Cow<'static, str> {
        match self {
            Self::Board => "#/board".into(),
            Self::Discover(req) => format!("#/discover/{}", resource_request_to_url_path(&req)).into(),
            Self::Detail => format!("#/detail/{}", "TODO").into(),
            Self::Player => "#/player".into(),
            Self::NotFound => "#/404".into(),
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        let hash = match url.hash {
            Some(hash) => hash,
            None => {
                match url.path.first().map(String::as_str) {
                    None | Some("") => return Self::Board,
                    _ => return Self::NotFound,
                }
            },
        };
        log!("HASH", hash);
        let mut hash = hash.split('/');
        // skip the part before the first `/`
        hash.next();

        match hash.next() {
            Some("") | Some("board") => Self::Board,
            Some("discover") => {
                let encoded_base = match hash.next() {
                    Some(base) => base,
                    None => {
                        error!("cannot find request base");
                        return Self::NotFound
                    },
                };

                let encoded_path = match hash.next() {
                    Some(base) => base,
                    None => {
                        error!("cannot find request path");
                        return Self::NotFound
                    },
                };

                let req = match resource_request_try_from_url_parts(encoded_base, encoded_path) {
                    Ok(req) => req,
                    Err(error) => {
                        error!(error);
                        return NotFound
                    }
                };

                Self::Discover(req)
            },
            Some("detail") => Self::Detail,
            Some("player") => Self::Player,
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Init
// ------ ------

fn resource_request_to_url_path(req: &ResourceRequest) -> String {
    let encoded_base = String::from(js_sys::encode_uri_component(&req.base));
    let encoded_path = String::from(js_sys::encode_uri_component(&req.path.to_string()));
    format!("{}/{}", encoded_base, encoded_path)
}

#[derive(Debug)]
enum ParseResourceRequestError {
    UriDecode(String),
    Resource(ParseResourceErr)
}

fn resource_request_try_from_url_parts(uri_encoded_base: &str, uri_encoded_path: &str) -> Result<ResourceRequest, ParseResourceRequestError> {
    let base: String = {
        js_sys::decode_uri_component(uri_encoded_base)
            .map_err(|_|  ParseResourceRequestError::UriDecode(uri_encoded_base.to_owned()))?
            .into()
    };

    let path: String = {
        js_sys::decode_uri_component(uri_encoded_path)
            .map_err(|_|  ParseResourceRequestError::UriDecode(uri_encoded_path.to_owned()))?
            .into()
    };

    Ok(ResourceRequest {
        base,
        path: ResourceRef::from_str(&path).map_err( ParseResourceRequestError::Resource)?
    })
}

fn default_resource_request() -> ResourceRequest {
    ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("catalog", "movie", "top"),
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    let model = Model {
        core: CoreModel::default(),
        page: url.into(),
        selected_meta_preview_id: None,
    };

    if let Some(msg) = handle_page_change(&model.page) {
        orders.send_msg(msg);
    }

    Init::new_with_url_handling(model, UrlHandling::None)
}

// ------ ------
//    Routes
// ------ ------

fn routes(url: Url) -> Option<Msg> {
//    log!("URL IN ROUTES", url);

    Some(Msg::RouteChanged(url))
}

fn handle_page_change(page: &Page) -> Option<Msg> {
    match page {
        Page::Discover(req) =>  Some(Msg::Core(Rc::new(CoreMsg::Action(Action::Load(ActionLoad::CatalogFiltered(req.clone())))))),
        _ => None
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    MetaPreviewClicked(MetaPreviewId),
    SetSelectorValues,
    RouteChanged(Url),
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
        },
        Msg::RouteChanged(url) => {
            model.page = url.into();
            if let Some(msg) = handle_page_change(&model.page) {
                orders.send_msg(msg);
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
    match model.page {
        Page::Board => page::board::view(),
        Page::Discover(_) => page::discover::view(&model),
        Page::Detail => page::detail::view(),
        Page::Player => page::player::view(),
        Page::NotFound => page::not_found::view()
    }
}



// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::build(init, update, view)
        .routes(routes)
        .build_and_start();
}
