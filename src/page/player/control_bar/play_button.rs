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
pub fn play_button(playing: bool) -> Node<Msg> {
    div![
        C!["control-bar-button", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(rem(4))
            .justify_content(CssJustifyContent::Center)
            .width(rem(4))
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => "-1",
            At::Title => if playing { "Pause" } else { "Play" },
        },
        ev(Ev::Click, |_| Msg::TogglePlay),
        if playing {
            pause_icon()
        } else {
            play_icon()
        }
    ]
}

#[view]
fn play_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, 100))
            .flex(CssFlex::None)
            .height(rem(2))
            .width(rem(3)),
        attrs!{
            At::ViewBox => "0 0 899 1024",
            At::from("icon") => "ic_play",
        },
        path![
            attrs!{
                At::D => "M891.482 512l-884.254 512v-1024z",
            }
        ],
    ]
}

#[view]
fn pause_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, 100))
            .flex(CssFlex::None)
            .height(rem(2))
            .width(rem(3)),
        attrs!{
            At::ViewBox => "0 0 899 1024",
            At::from("icon") => "ic_pause",
        },
        path![
            attrs!{
                At::D => "M0 0h268.047v1024h-268.047v-1024z",
            }
        ],
        path![
            attrs!{
                At::D => "M540.311 0h268.047v1024h-268.047v-1024z",
            }
        ],
    ]
}
