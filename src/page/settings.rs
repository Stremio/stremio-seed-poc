use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    context: &mut Context,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();

    let model = model.get_or_insert_with(move || Model {
        active_side_menu_button: SideMenuButton::General
    });
    Some(PageId::Settings)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    active_side_menu_button: SideMenuButton 
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SideMenuButton {
    General,
    Player,
    StreamingServer,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    basic_layout(BasicLayoutArgs {
        page_content: settings_content(model, context).map_msg(msg_mapper),
        container_class: "settings-container",
        context,
        page_id,
        search_args: None,
    })
}

#[view]
fn settings_content<'a>(model: &Model, context: &Context) -> Node<Msg> {
    div![
        C!["settings-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(pc(100))
            .width(pc(100)),            
        side_menu_container(model.active_side_menu_button),
        sections_container(),
    ]
}

#[view]
fn side_menu_container(active_button: SideMenuButton) -> Node<Msg> {
    let app_version = "5.0.0";
    div![
        C!["side-menu-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Column)
            .padding(rem(3))
            .width(rem(20)),
        side_menu_button("General", active_button == SideMenuButton::General),
        side_menu_button("Player", active_button == SideMenuButton::Player),
        side_menu_button("Streaming server", active_button == SideMenuButton::StreamingServer),
        div![
            C!["spacing"],
            s()
                .flex("1"),
        ],
        div![
            C!["version-info-label"],
            s()
                .color(Color::SecondaryVariant1_90)
                .flex("0 1 auto")
                .margin("0.5rem 0"),
            attrs!{
                At::Title => app_version,
            },
            "App Version: ",
            app_version,
        ]
    ]
}

#[view]
fn side_menu_button(title: &str, active: bool) -> Node<Msg> {
    div![
        C!["side-menu-button", IF!(active => "selected"), "button-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .color(Color::SecondaryVariant1_90)
            .flex(CssFlex::None)
            .font_size(rem(1.1))
            .padding(rem(1))
            .cursor(CssCursor::Pointer),
        IF!(active => {
            s()
                .background_color(Color::Background)
                .color(Color::SurfaceLight5_90)
        }),
        s()
            .hover()
            .background_color(Color::BackgroundLight1),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
            At::from("data-section") => title,
        },
        title,
    ]
}

#[view]
fn sections_container() -> Node<Msg> {
    div![
        C!["sections-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        general_section(),
        player_section(),
        streaming_server_section(),
    ]
}

#[view]
fn general_section() -> Node<Msg> {
    section_container("General", true)
}

#[view]
fn player_section() -> Node<Msg> {
    section_container("Player", true)
}

#[view]
fn streaming_server_section() -> Node<Msg> {
    section_container("Streaming Server", false)
}

#[view]
fn section_container(title: &str, bottom_border: bool) -> Node<Msg> {
    div![
        C!["section-container"],
        IF!(bottom_border => {
            s()
                .border_bottom("thin solid hsla(224.3,42.1%,66%,0.9)")
        }),
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .overflow(CssOverflow::Visible)
            .padding("3rem 0"),
        div![
            C!["section-title"],
            s()
                .align_self(CssAlignSelf::Stretch)
                .color(Color::SurfaceLight5_90)
                .flex(CssFlex::None)
                .font_size(rem(1.8))
                .line_height(rem(3.4))
                .margin_bottom(rem(1)),
            title,
        ],
    ]
}

#[view]
fn option_container() -> Node<Msg> {
    div![
        
    ]
}
