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

mod play_button;
use play_button::play_button;

mod mute_button;
use mute_button::mute_button;

mod volume_slider;
use volume_slider::volume_slider;

#[view]
pub fn control_bar(playing: bool, muted: bool, volume: u32) -> Node<Msg> {
    div![
        C!["layer", "control-bar-layer", "control-bar-container"],
        s()
            .overflow(CssOverflow::Visible)
            .top("initial")
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .z_index("0")
            .padding("0 1.5rem"),
        s()
            .before()
            .bottom("0")
            .box_shadow("0 0 8rem 8rem hsl(0deg 0% 0%)")
            .content(r#""""#)
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .z_index("-1"),
        seek_bar(),
        control_bar_buttons(playing, muted, volume),
    ]
}

#[view]
fn seek_bar() -> Node<Msg> {
    div![
        
    ]
}

#[view]
fn control_bar_buttons(playing: bool, muted: bool, volume: u32) -> Node<Msg> {
    div![
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        play_button(playing),
        mute_button(muted, volume),
        volume_slider(volume),
    ]
}
