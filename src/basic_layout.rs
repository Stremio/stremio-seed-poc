use crate::{PageId, Msg, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use std::rc::Rc;

mod menu;
mod horizontal_nav_bar;
mod vertical_nav_bar;

use horizontal_nav_bar::horizontal_nav_bar;
use vertical_nav_bar::vertical_nav_bar;

pub struct BasicLayoutArgs<'a> {
    pub page_content: Node<Msg>,
    pub container_class: &'a str,
    pub context: &'a Context,
    pub page_id: PageId,
    pub search_args: Option<SearchArgs<'a>>,
    pub modal: Option<Node<Msg>>,
}

pub struct SearchArgs<'a> {
    pub input_search_query: &'a str,
    pub on_search_query_input_changed: Rc<dyn Fn(String) -> Msg>,
    pub on_search: Rc<dyn Fn() -> Msg>,
}

#[view]
pub fn basic_layout(mut args: BasicLayoutArgs) -> Vec<Node<Msg>> {
    let modal = args.modal.take();
    vec![
        route_content(args),
        // @TODO allow Option<Node<Msg>> in Seed
        if let Some(modal) = modal {
            modals_container(modal)
        } else {
            empty![]
        }
    ]
}

#[view]
fn route_content(args: BasicLayoutArgs) -> Node<Msg> {
    div![
        C!["route-content"],
        s()
            .position(CssPosition::Absolute)
            .bottom("0")
            .left("0")
            .right("0")
            .top("0")
            .overflow(CssOverflow::Hidden)
            .z_index("0"),
        div![
            C![args.container_class, "main-nav-bars-container"],
            s()
                .background_color(Color::BackgroundDark2)
                .height(pc(100))
                .width(pc(100))
                .position(CssPosition::Relative)
                .z_index("0"),
            horizontal_nav_bar(
                &args.context.root_base_url, 
                args.search_args.as_ref(), 
                args.context.menu_visible,
                args.context.fullscreen,
                args.context.core_model.ctx.profile.auth.as_ref().map(|auth| &auth.user),
            ),
            vertical_nav_bar(&args.context.root_base_url, args.page_id),
            nav_content_container(args.page_content),
        ]
    ]
}

#[view]
fn nav_content_container(page_content: Node<Msg>) -> Node<Msg> {
    div![
        C!["nav-content-container"],
        s()
            .bottom("0")
            .left(global::VERTICAL_NAV_BAR_SIZE)
            .position(CssPosition::Absolute)
            .right("0")
            .top(global::HORIZONTAL_NAV_BAR_SIZE)
            .z_index("0"),
        page_content,
    ]
}

#[view]
fn modals_container(modal: Node<Msg>) -> Node<Msg> {
    div![
        C!["modals-container"],
        s()
            .height("0")
            .width("0"),
        modal,
    ]
}


