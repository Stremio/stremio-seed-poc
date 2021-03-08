use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::types::profile::User;
use crate::Urls as RootUrls;
use crate::styles::{self, themes::Color, global};
use crate::page::settings::Msg;
use crate::page::settings::section::{
    option,
    control::{label, dropdown, connect_button, link_label}
};

mod control;

mod general;
use general::general_section;

mod player;
use player::player_section;

mod streaming_server;
use streaming_server::streaming_server_section;

#[view]
pub fn sections(root_base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["sections-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        general_section(root_base_url, user),
        player_section(),
        streaming_server_section(),
    ]
}

#[view]
pub fn section(title: &str, bottom_border: bool, options: Vec<Node<Msg>>) -> Node<Msg> {
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
        options,
    ]
}

#[view]
pub fn section_option(extra_style: Option<Style>, content: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        C!["option-container"],
        s()
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .margin_bottom(rem(2))
            .max_width(rem(35))
            .overflow(CssOverflow::Visible),
        extra_style,
        content,
    ]
}
