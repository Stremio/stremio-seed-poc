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
pub fn control_bar() -> Node<Msg> {
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
    ]
}
