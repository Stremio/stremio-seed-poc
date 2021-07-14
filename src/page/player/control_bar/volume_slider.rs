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
pub fn volume_slider(volume: u32, active: bool) -> Node<Msg> {
    div![
        C!["volume-slider", "slider-container", IF!(active => "active")],
        s()
            .flex("0 1 16rem")
            .height(rem(4))
            .margin("0 1rem")
            .min_width(rem(10))
            .align_items(CssAlignItems::Center)
            .cursor(CssCursor::Pointer)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .z_index("0"),
        layer(track()),
        layer(track_before(volume)),
        layer(thumb(volume)),
        mouse_ev(Ev::MouseDown, |event| {
            Msg::ActivateVolumeSlider(get_volume(event))
        }),
        active.then(|| {
            mouse_ev(Ev::MouseMove, |event| {
                Msg::VolumeSliderMoved(get_volume(event))
            })
        }),
        ev(Ev::MouseUp, |_| Msg::DeactivateVolumeSlider),
        ev(Ev::MouseLeave, |_| Msg::DeactivateVolumeSlider),
    ]
}

fn get_volume(event: web_sys::MouseEvent) -> u32 {
    let offset = event.offset_x();
    let width = event.target().unwrap().unchecked_into::<web_sys::Element>().client_width();
    let volume = offset as f32 / width as f32 * 100.;
    volume as u32
}

#[view]
pub fn layer(content: Node<Msg>) -> Node<Msg> {
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
        content
    ]
}

#[view]
pub fn track() -> Node<Msg> {
    div![
        C!["track"],
        s()
            .background_color(hsl(0, 0, 50))
            .flex("1")
            .height(global::TRACK_SIZE),
    ]
}

#[view]
pub fn track_before(volume: u32) -> Node<Msg> {
    div![
        C!["track"],
        s()
            .width(format!("calc({}%)", volume).as_str())
            .background_color(hsl(0, 0, 90))
            .flex(CssFlex::None)
            .height(global::TRACK_SIZE),
    ]
}

#[view]
fn thumb(volume: u32) -> Node<Msg> {
    svg![
        C!["thumb"],
        s()
            .margin_left(format!("calc({}%)", volume).as_str())
            .fill(hsl(0, 0, 100))
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
