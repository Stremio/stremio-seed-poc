#![allow(
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

// @TODO remove
#![allow(
    dead_code,
    unused_imports,
)]

mod multi_select;
mod page;
mod styles;
mod env;

use env::WebEnv;
use futures::compat::Future01CompatExt;
use seed::{prelude::*, *};
use seed_styles::pc;
use seed_styles::*;
use std::rc::Rc;
use stremio_core::models::{ctx::Ctx, meta_details::MetaDetails};
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::installed_addons_with_filters::InstalledAddonsWithFilters;
use stremio_core::runtime::{Update, msg::Msg as CoreMsg, Effect};
use stremio_core::types::addon::DescriptorPreview;
use stremio_core::types::resource::MetaItemPreview;
use stremio_derive::Model;
use seed_hooks::{*, topo::nested as view};

// ---- url parts ----

const BOARD: &str = "board";
const DISCOVER: &str = "discover";
const DETAIL: &str = "metadetails";
const PLAYER: &str = "player";
const ADDONS: &str = "addons";
const SEARCH: &str = "search";
const TEST_LINKS: &str = "test_links";

// ------ ------
//    Actions
// ------ ------

#[derive(Clone)]
pub enum Actions {
    UpdateCoreModel(Rc<CoreMsg>)
}

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    styles::global::init();

    let root_base_url = url.to_hash_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(|Actions::UpdateCoreModel(core_msg)| Msg::CoreMsg(core_msg))
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
        search_model: None,
    }
}

// ------ ------
//     Model
// ------ ------

#[allow(clippy::large_enum_variant)]
pub struct Model {
    context: Context,
    page_id: Option<PageId>,
    // ---- page models ----
    detail_model: Option<page::detail::Model>,
    discover_model: Option<page::discover::Model>,
    addons_model: Option<page::addons::Model>,
    search_model: Option<page::search::Model>,
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
    Search,
    TestLinks,
}

// ------ CoreModel  ------

#[derive(Model, Default)]
struct CoreModel {
    ctx: Ctx<WebEnv>,
    catalog: CatalogWithFilters<MetaItemPreview>,
    addon_catalog: CatalogWithFilters<DescriptorPreview>,
    installed_addons: InstalledAddonsWithFilters,
    meta_details: MetaDetails,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
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
    pub fn search_urls(self) -> page::search::Urls<'a> {
        page::search::Urls::new(self.base_url().add_hash_path_part(SEARCH))
    }
    pub fn test_links(self) -> Url {
        self.base_url().add_hash_path_part(TEST_LINKS)
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
enum Msg {
    UrlChanged(subs::UrlChanged),
    CoreMsg(Rc<CoreMsg>),
    HandleEffectMsg(Rc<CoreMsg>),
    DiscoverMsg(page::discover::Msg),
    DetailMsg(page::detail::Msg),
    AddonsMsg(page::addons::Msg),
    SearchMsg(page::search::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            let page_id = match url.next_hash_path_part() {
                None => Some(PageId::Board),
                Some(DISCOVER) => page::discover::init(
                    url,
                    &mut model.discover_model,
                    &mut orders.proxy(Msg::DiscoverMsg),
                ),
                Some(DETAIL) => page::detail::init(
                    url,
                    &mut model.detail_model,
                    &mut orders.proxy(Msg::DetailMsg),
                ),
                Some(PLAYER) => Some(PageId::Player),
                Some(ADDONS) => page::addons::init(
                    url,
                    &mut model.addons_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::AddonsMsg),
                ),
                Some(SEARCH) => page::search::init(
                    url,
                    &mut model.search_model,
                    &mut orders.proxy(Msg::SearchMsg),
                ),
                Some(TEST_LINKS) => Some(PageId::TestLinks),
                _ => None,
            };
            model.page_id = page_id.or(Some(PageId::NotFound));
        }
        Msg::CoreMsg(core_msg) => {
            let effects = model.context.core_model.update(&core_msg);

            if !effects.has_changed {
                orders.skip();
            }

            for effect in effects {
                match effect {
                    Effect::Msg(core_msg) => {
                        orders.send_msg(Msg::HandleEffectMsg(Rc::new(core_msg)));
                    }
                    Effect::Future(cmd) => {
                        orders.perform_cmd(async move {
                            Msg::HandleEffectMsg(Rc::new(cmd.await))
                        });
                    }
                }
            }
        }
        Msg::HandleEffectMsg(core_msg) => {
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
                    &mut orders.proxy(Msg::AddonsMsg),
                );
            }
        }
        Msg::SearchMsg(page_msg) => {
            if let Some(page_model) = &mut model.search_model {
                page::search::update(page_msg, page_model, &mut orders.proxy(Msg::SearchMsg));
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["router", "routes-container"],
        s()
            .width(pc(100))
            .height(pc(100))
            .position(CssPosition::Relative)
            .z_index("0"),
        div![
            C!["route-container",],
            s()
                .position(CssPosition::Absolute)
                .top("0")
                .right("0")
                .bottom("0")
                .left("0")
                .z_index("0"),
            model.page_id.map(|page_id| {
                match page_id {
                    PageId::Board => page::board::view(&model.context.root_base_url).into_nodes(),
                    PageId::Detail => page::detail::view(&model.context).into_nodes(),
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
                            page::addons::view(page_model, &model.context)
                                .map_msg(Msg::AddonsMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
                    PageId::Search => {
                        if let Some(page_model) = &model.search_model {
                            page::search::view(page_model, &model.context)
                                .map_msg(Msg::SearchMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    },
                    PageId::TestLinks => page::test_links::view(&model.context.root_base_url).into_nodes(),
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
