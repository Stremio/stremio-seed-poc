use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
use enclose::enc;
use serde::Serialize;
use crate::{PageId, Context, Actions, Events};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::types::resource::{Stream, StreamSource};
use stremio_core::models::player::Selected as PlayerSelected;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg, Internal};
use super::Msg;

#[view]
pub fn seek_bar(active: bool, video_position: u32, video_length: u32) -> Node<Msg> {
    div![
        C!["seek-bar", "seek-bar-container"],
        s()
            .height(rem(2.5))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        s()
            .style_other(":hover .thumb")
            .fill(hsl(0, 0, 100)),
        s()
            .style_other(":hover .track-before")
            .background_color(Color::PrimaryLight5),
        label("00:00:51"),
        slider(),
        label("00:03:19"),
    ]
}

#[view]
fn label(text: &str) -> Node<Msg> {
    div![
        C!["label"],
        s()
            .color(hsl(0, 0, 100))
            .direction(CssDirection::Rtl)
            .flex(CssFlex::None)
            .max_width(rem(5))
            .text_align(CssTextAlign::Left)
            .text_overflow("ellipsis")
            .white_space(CssWhiteSpace::NoWrap),
        text,
    ]
}

#[view]
fn slider() -> Node<Msg> {
    let position_percent = 26;
    div![
        C!["slider", "slider-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .margin(format!("0 {}", global::THUMB_SIZE).as_str())
            .cursor(CssCursor::Pointer)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .z_index("0"),
        layer(track()),
        layer(track_before(position_percent)),
        layer(thumb(position_percent)),
    ]
}

#[view]
fn layer(content: Node<Msg>) -> Node<Msg> {
    div![
        C!["layer"],
        s()
            .align_items(CssAlignItems::Center)
            .bottom("0")
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .left("0")
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
        content,
    ]
}

#[view]
pub fn track() -> Node<Msg> {
    div![
        C!["track"],
        s()
            .background_color(Color::PrimaryDark3)
            .flex("1")
            .height(global::TRACK_SIZE),
    ]
}

#[view]
pub fn track_before(position_percent: u32) -> Node<Msg> {
    div![
        C!["track-before"],
        s()
            .width(format!("calc({}%)", position_percent).as_str())
            .background_color(Color::PrimaryLight3)
            .flex(CssFlex::None)
            .height(global::TRACK_SIZE),
    ]
}

#[view]
fn thumb(position_percent: u32) -> Node<Msg> {
    svg![
        C!["thumb"],
        s()
            .margin_left(format!("calc({}%)", position_percent).as_str())
            .fill("transparent")
            .flex(CssFlex::None)
            .height(global::THUMB_SIZE)
            .transform("translateX(-50%)")
            .width(global::THUMB_SIZE)
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 10 10",
        },
        circle![
            attrs!{
                At::Cx => 5,
                At::Cy => 5,
                At::R => 5,
            }
        ],
    ]
}
