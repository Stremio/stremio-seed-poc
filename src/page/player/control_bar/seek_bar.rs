use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::borrow::Cow;
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
pub fn seek_bar(active: bool, time: Option<u32>, duration: Option<u32>) -> Node<Msg> {
    let position_percent = match (time, duration) {
        (Some(time), Some(duration)) => time as f32 / duration as f32 * 100.,
        _ => 0.,
    };
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
        label(format_time(time)),
        slider(active, position_percent, duration),
        label(format_time(duration)),
    ]
}

fn format_time(seconds: Option<u32>) -> Cow<'static, str> {
    let seconds = match seconds {
        None => return "--:--:--".into(),
        Some(seconds) => seconds,
    }; 
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds).into()
}

#[view]
fn label(text: Cow<'static, str>) -> Node<Msg> {
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
fn slider(active: bool, position_percent: f32, duration: Option<u32>) -> Node<Msg> {
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
        mouse_ev(Ev::MouseDown, move |event| {
            Msg::ActivateSeekSlider(get_time(event, duration))
        }),
        active.then(|| {
            mouse_ev(Ev::MouseMove, move |event| {
                Msg::SeekSliderMoved(get_time(event, duration))
            })
        }),
        ev(Ev::MouseUp, |_| Msg::DeactivateSeekSlider),
        ev(Ev::MouseLeave, |_| Msg::DeactivateSeekSlider),
    ]
}

fn get_time(event: web_sys::MouseEvent, duration: Option<u32>) -> u32 {
    let duration = duration.unwrap_or_default() as f32;
    let offset = event.offset_x();
    let width = event.target().unwrap().unchecked_into::<web_sys::Element>().client_width();
    let time = offset as f32 / width as f32 * duration;
    time as u32
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
            .z_index("0")
            .pointer_events("none"),
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
pub fn track_before(position_percent: f32) -> Node<Msg> {
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
fn thumb(position_percent: f32) -> Node<Msg> {
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
