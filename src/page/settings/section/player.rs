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
    section_option,
    section,
    control::{label, dropdown, connect_button, link_label, color_picker, checkbox}
};
use web_sys::HtmlElement;

#[view]
pub fn player_section(section_ref: &ElRef<HtmlElement>) -> Node<Msg> {
    let options = vec![
        section_option(None, vec![
            label("Subtitles language"),
            dropdown("English")
        ]),
        section_option(None, vec![
            label("Subtitles size"),
            dropdown("100%")
        ]),
        section_option(None, vec![
            label("Subtitles text color"),
            color_picker(Some(("rgb(255, 255, 255)", "#FFFFFFFF"))),
        ]),
        section_option(None, vec![
            label("Subtitles background color"),
            color_picker(None),
        ]),
        section_option(None, vec![
            label("Subtitles outline color"),
            color_picker(None),
        ]),
        section_option(None, vec![
            label("Auto-play next episode"),
            checkbox(false, None, true),
        ]),
        section_option(None, vec![
            label("Play in background"),
            checkbox(true, None, false),
        ]),
        section_option(None, vec![
            label("Play in external player"),
            checkbox(false, None, false),
        ]),
        section_option(Some(s().margin_bottom("0")), vec![
            label("Hardware-accelerated decoding"),
            checkbox(false, None, false),
        ]),
    ];
    section("Player", true, section_ref, options)
}
