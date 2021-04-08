use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::types::profile::User;
use crate::Urls as RootUrls;
use crate::styles::{self, themes::Color, global};
use crate::page::settings::Msg;
use crate::page::settings::section::Section;

#[view]
pub fn side_menu(active_section: Section) -> Node<Msg> {
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
        side_menu_button(
            "General", 
            active_section == Section::General, 
            || Msg::MenuButtonClicked(Section::General)
        ),
        side_menu_button(
            "Player", 
            active_section == Section::Player, 
            || Msg::MenuButtonClicked(Section::Player)
        ),
        side_menu_button(
            "Streaming server", 
            active_section == Section::StreamingServer, 
            || Msg::MenuButtonClicked(Section::StreamingServer)
        ),
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
fn side_menu_button(title: &str, active: bool, on_click: fn() -> Msg) -> Node<Msg> {
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
        ev(Ev::Click, move |_| on_click()),
        title,
    ]
}
