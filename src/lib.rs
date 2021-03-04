#![allow(
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::wildcard_imports
)]

// @TODO remove
#![allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
)]

mod basic_layout;
mod multi_select;
mod page;
mod styles;
mod env;

use env::WebEnv;
use futures::compat::Future01CompatExt;
use seed::{prelude::*, *};
use seed_styles::pc;
use seed_styles::*;
use core::future;
use std::rc::Rc;
use stremio_core::models::{ctx::Ctx, meta_details::MetaDetails};
use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::installed_addons_with_filters::InstalledAddonsWithFilters;
use stremio_core::runtime::{Update, Effect};
use stremio_core::runtime::{Env, EnvError};
use stremio_core::runtime::msg::{Msg as CoreMsg, Event, CtxStorageResponse};
use stremio_core::types::addon::DescriptorPreview;
use stremio_core::types::resource::MetaItemPreview;
use stremio_core::types::profile::Profile;
use stremio_core::types::library::LibraryBucket;
use stremio_core::constants::{
    LIBRARY_RECENT_STORAGE_KEY, LIBRARY_STORAGE_KEY, PROFILE_STORAGE_KEY,
};
use stremio_derive::Model;
use seed_hooks::{*, topo::nested as view};

// ---- url parts ----

const DISCOVER: &str = "discover";
const DETAIL: &str = "metadetails";
const INTRO: &str = "intro";
const LIBRARY: &str = "library";
const PLAYER: &str = "player";
const ADDONS: &str = "addons";
const SEARCH: &str = "search";
const SETTINGS: &str = "settings";
const TEST_LINKS: &str = "test_links";

// ------ ------
//    Actions
// ------ ------

#[derive(Clone)]
pub enum Actions {
    UpdateCoreModel(Rc<CoreMsg>),
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
        .stream(streams::window_event(Ev::Click, |_| Msg::WindowClicked))
        .notify(subs::UrlChanged(url))
        .perform_cmd(async { Msg::CtxStorageResponse(futures::try_join!(
            WebEnv::get_storage::<Profile>(PROFILE_STORAGE_KEY),
            WebEnv::get_storage::<LibraryBucket>(LIBRARY_RECENT_STORAGE_KEY),
            WebEnv::get_storage::<LibraryBucket>(LIBRARY_STORAGE_KEY),
        ))});
        // @TODO listen for `fullscreenchange` once it's implemented in Safari

    Model {
        context: Context {
            core_model: CoreModel::default(),
            root_base_url,
            menu_visible: false,
            fullscreen: false,
        },
        page_id: None,
        // ---- page models ----
        board_model: None,
        detail_model: None,
        intro_model: None,
        library_model: None,
        discover_model: None,
        addons_model: None,
        search_model: None,
        settings_model: None,
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
    board_model: Option<page::board::Model>,
    detail_model: Option<page::detail::Model>,
    intro_model: Option<page::intro::Model>,
    library_model: Option<page::library::Model>,
    discover_model: Option<page::discover::Model>,
    addons_model: Option<page::addons::Model>,
    search_model: Option<page::search::Model>,
    settings_model: Option<page::settings::Model>,
}

// ------ Context ------

pub struct Context {
    core_model: CoreModel,
    root_base_url: Url,
    menu_visible: bool,
    fullscreen: bool,
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageId {
    Board,
    Detail,
    Discover,
    Intro,
    Library,
    Player,
    Addons,
    NotFound,
    Search,
    Settings,
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
    pub fn intro(self) -> Url {
        self.base_url().add_hash_path_part(INTRO)
    }
    pub fn library(self) -> Url {
        self.base_url().add_hash_path_part(LIBRARY)
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
    pub fn settings(self) -> Url {
        self.base_url().add_hash_path_part(SETTINGS)
    }
    pub fn test_links(self) -> Url {
        self.base_url().add_hash_path_part(TEST_LINKS)
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum Msg {
    CtxStorageResponse(Result<CtxStorageResponse, EnvError>),
    UrlChanged(subs::UrlChanged),
    CoreMsg(Rc<CoreMsg>),
    HandleEffectMsg(Rc<CoreMsg>),
    GoToSearchPage,
    ToggleMenu,
    HideMenu,
    WindowClicked,
    BoardMsg(page::board::Msg),
    DiscoverMsg(page::discover::Msg),
    DetailMsg(page::detail::Msg),
    IntroMsg(page::intro::Msg),
    LibraryMsg(page::library::Msg),
    AddonsMsg(page::addons::Msg),
    SearchMsg(page::search::Msg),
    SettingsMsg(page::settings::Msg),
    ToggleFullscreen,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CtxStorageResponse(Ok((profile, recent_bucket, other_bucket))) => {
            let ctx = &mut model.context.core_model.ctx;
            if let Some(profile) = profile {
                ctx.profile = profile;
            }
            if let Some(recent_bucket) = recent_bucket {
                ctx.library.merge_bucket(recent_bucket);
            };
            if let Some(other_bucket) = other_bucket {
                ctx.library.merge_bucket(other_bucket);
            };
        }
        Msg::CtxStorageResponse(Err(error)) => {
            log!(error.message());
        }
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            let page_id = match url.next_hash_path_part() {
                None => page::board::init(
                    url,
                    &mut model.board_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::BoardMsg),
                ),
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
                Some(INTRO) => page::intro::init(
                    url,
                    &mut model.intro_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::IntroMsg),
                ),
                Some(LIBRARY) => page::library::init(
                    url,
                    &mut model.library_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::LibraryMsg),
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
                Some(SETTINGS) => page::settings::init(
                    url,
                    &mut model.settings_model,
                    &mut model.context,
                    &mut orders.proxy(Msg::SettingsMsg),
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
            if let CoreMsg::Event(Event::UserLoggedOut {..}) = core_msg.as_ref() {
                orders.request_url(Urls::new(&model.context.root_base_url).root());
            }
            orders.notify(core_msg);
        }
        Msg::GoToSearchPage => {
            orders.request_url(Urls::new(&model.context.root_base_url).search_urls().root());
        }
        Msg::ToggleMenu => {
            model.context.menu_visible = !model.context.menu_visible;
        }
        Msg::HideMenu => {
            model.context.menu_visible = false;
        }
        Msg::WindowClicked => {
            if not(model.context.menu_visible) {
                orders.skip();
                return;
            }
            model.context.menu_visible = false;
        }
        Msg::BoardMsg(page_msg) => {
            if let Some(page_model) = &mut model.board_model {
                page::board::update(
                    page_msg, 
                    page_model, 
                    &mut model.context,
                    &mut orders.proxy(Msg::BoardMsg)
                );
            }
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
        Msg::IntroMsg(page_msg) => {
            if let Some(page_model) = &mut model.intro_model {
                page::intro::update(
                    page_msg, 
                    page_model, 
                    &mut model.context, 
                    &mut orders.proxy(Msg::IntroMsg)
                );
            }
        }
        Msg::LibraryMsg(page_msg) => {
            if let Some(page_model) = &mut model.library_model {
                page::library::update(
                    page_msg,
                    page_model,
                    &mut orders.proxy(Msg::LibraryMsg),
                );
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
                page::search::update(
                    page_msg, 
                    page_model, 
                    &mut model.context,
                    &mut orders.proxy(Msg::SearchMsg)
                );
            }
        }
        Msg::SettingsMsg(page_msg) => {
            if let Some(page_model) = &mut model.settings_model {
                page::settings::update(
                    page_msg,
                    page_model,
                    &mut orders.proxy(Msg::SettingsMsg),
                );
            }
        }
        Msg::ToggleFullscreen => {
            if model.context.fullscreen {
                close_fullscreen();
                model.context.fullscreen = false;
            } else {
                open_fullscreen();
                model.context.fullscreen = true;
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
                    PageId::Board => if let Some(page_model) = &model.board_model {
                        page::board::view(page_model, &model.context, page_id, Msg::BoardMsg)
                            .into_nodes()
                    } else {
                        vec![]
                    },
                    PageId::Detail => page::detail::view(&model.context).into_nodes(),
                    PageId::Discover => {
                        if let Some(page_model) = &model.discover_model {
                            page::discover::view(page_model, &model.context, page_id, Msg::DiscoverMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
                    PageId::Intro => if let Some(page_model) = &model.intro_model {
                        page::intro::view(page_model)
                            .map_msg(Msg::IntroMsg)
                            .into_nodes()
                    } else {
                        vec![]
                    },
                    PageId::Library => {
                        if let Some(page_model) = &model.library_model {
                            page::library::view(page_model, &model.context, page_id, Msg::LibraryMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
                    PageId::Player => page::player::view().into_nodes(),
                    PageId::Addons => {
                        if let Some(page_model) = &model.addons_model {
                            page::addons::view(page_model, &model.context, page_id, Msg::AddonsMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
                    PageId::Search => {
                        if let Some(page_model) = &model.search_model {
                            page::search::view(page_model, &model.context, page_id, Msg::SearchMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    },
                    PageId::Settings => {
                        if let Some(page_model) = &model.settings_model {
                            page::settings::view(page_model, &model.context, page_id, Msg::SettingsMsg)
                                .into_nodes()
                        } else {
                            vec![]
                        }
                    }
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

// ------ ------
//    Extern
// ------ ------

#[wasm_bindgen(module = "/js/fullscreen.js")]
extern "C" {
    #[wasm_bindgen(js_name = openFullscreen)]
    fn open_fullscreen();
    #[wasm_bindgen(js_name = closeFullscreen)]
    fn close_fullscreen();
}
