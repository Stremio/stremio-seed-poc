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
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use std::rc::Rc;
use stremio_core::state_types::{CatalogFiltered, Ctx, Msg as CoreMsg, Update};
use stremio_core::types::addons::DescriptorPreview;
use stremio_core::types::MetaPreview;
use stremio_derive::Model;
use seed_hooks::*;
use seed_hooks::state_access::CloneState;

// ---- url parts ----

const BOARD: &str = "board";
const DISCOVER: &str = "discover";
const DETAIL: &str = "detail";
const PLAYER: &str = "player";
const ADDONS: &str = "addons";

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
    init_styles();

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
    }
}

fn init_styles() {
    load_app_themes(&[default_breakpoint_theme, default_scale_theme]);

    GlobalStyle::default()
    .style(
        "body", // @TODO: should be "html" once possible
        s()
            .font_size(px(16))
            .box_sizing(CssBoxSizing::BorderBox)
            .raw("text-rendering: optimizeLegibility;")
            .raw(format!("-webkit-text-size-adjust: {};", pc(100)).as_str())
            .raw(format!("-moz-text-size-adjust: {};", pc(100)).as_str())
    )
    .style(
        "body",
        s()
            .color("#4a4a4a")
            .font_size(em(1))
            .font_weight("400")
            .line_height("1.5")
    )
    .style(
        "body, button, input, select, textarea", 
        s()
            .font_family(r#"BlinkMacSystemFont,-apple-system,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Fira Sans","Droid Sans","Helvetica Neue",Helvetica,Arial,sans-serif"#)
    )
    .style(
        "*, ::after, ::before",
        s()
            .box_sizing(CssBoxSizing::Inherit)
    )
    .style(
        "a",
        s()
            .color("#3273dc")
            .cursor(CssCursor::Pointer)
            .text_decoration(CssTextDecoration::None)
    )
    .style(
        "a",
        s()
            .hover()
            .color("#363636")
    )
    .style(
        "span",
        s()
            .font_style(CssFontStyle::Inherit)
            .font_weight(CssFontWeight::Inherit)
    )
    .style(
        "blockquote, body, dd, dl, dt, fieldset, figure, h1, h2, h3, h4, h5, h6, hr, html, iframe, legend, li, ol, p, pre, textarea, ul",
        s()
            .m("0")
            .p("0")
    )
    .activate_init_styles();
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Breakpoint {
    // basic
    Mobile,
    Tablet,
    Desktop,
    WideScreen,
    FullHD,
    // extra
    TabletOnly,
    Touch,
    DesktopOnly,
    WideScreenOnly,
}
impl BreakpointTheme for Breakpoint {} 

fn default_breakpoint_theme() -> Theme {
    use Breakpoint::*;
    Theme::new("default_breakpoint_theme")
        // basic
        .set_breakpoint(Mobile, (0, Some(769))) 
        .set_breakpoint(Tablet, (769, Some(1024)))
        .set_breakpoint(Desktop, (1024, Some(1216)))
        .set_breakpoint(WideScreen, (1216, Some(1408)))
        .set_breakpoint(FullHD, (1408, None))
        .breakpoint_scale([769, 1024, 1216, 1408]) 
        // extra
        .set_breakpoint(TabletOnly, (769, Some(1024)))
        .set_breakpoint(Touch, (0, Some(1024)))
        .set_breakpoint(DesktopOnly, (1024, Some(1216)))
        .set_breakpoint(WideScreenOnly, (1216, Some(1408)))
}

fn default_scale_theme() -> Theme {
    Theme::new("default_scale_theme")
        .font_size_scale(&[rem(3), rem(2.5), rem(2), rem(1.5), rem(1.25), rem(1), rem(0.75)])
}

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
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            let page_id = match url.next_hash_path_part() {
                None | Some(BOARD) => Some(PageId::Board),
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
                    &mut orders.proxy(Msg::AddonsMsg),
                ),
                _ => None,
            };
            model.page_id = page_id.or(Some(PageId::NotFound));
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

// ------ ------
//     View
// ------ ------

// #[topo::nested]
fn view(model: &Model) -> Node<Msg> {
    let dummy_text = use_state(|| "remove me");
    log!(dummy_text.get());

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
