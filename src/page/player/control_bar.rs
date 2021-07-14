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

mod network_button;
use network_button::network_button;

mod info_button;
use info_button::info_button;

mod screencast_button;
use screencast_button::screencast_button;

mod subtitles_button;
use subtitles_button::subtitles_button;

mod videos_button;
use videos_button::videos_button;

mod seek_bar;
use seek_bar::seek_bar;

#[view]
pub fn control_bar(
    playing: bool, 
    muted: bool, 
    volume: u32, 
    active_volume_slider: bool, 
    active_seek_bar: bool, 
    video_position: u32, 
    video_length: u32
) -> Node<Msg> {
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
        seek_bar(active_seek_bar, video_position, video_length),
        control_bar_buttons(playing, muted, volume, active_volume_slider),
    ]
}

#[view]
fn control_bar_buttons(playing: bool, muted: bool, volume: u32, active_volume_slider: bool) -> Node<Msg> {
    div![
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        play_button(playing),
        mute_button(muted, volume),
        volume_slider(volume, active_volume_slider),
        spacer(),
        network_button(),
        info_button(),
        screencast_button(),
        subtitles_button(),
        videos_button(),
    ]
}

#[view]
fn spacer() -> Node<Msg> {
    div![
        C!["spacing"],
        s()
            .flex("1"), 
    ]
}
