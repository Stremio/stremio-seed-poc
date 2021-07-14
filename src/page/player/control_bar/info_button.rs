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
pub fn info_button() -> Node<Msg> {
    div![
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(rem(4))
            .justify_content(CssJustifyContent::Center)
            .width(rem(4))
            .cursor(CssCursor::Pointer),
        C!["control-bar-button", "button-container"],
        attrs!{
            At::TabIndex => -1,
        },
        ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); }),
        icon(),
    ]
}

#[view]
pub fn icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, 100))
            .flex(CssFlex::None)
            .height(rem(2))
            .width(rem(3))
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 1025 1024",
            At::from("icon") => "ic_info",
        },
        path![
            attrs!{
                At::D => "M883.351 161.732c-90.543-95.747-216.891-156.82-357.52-161.708l-0.88-0.024c-3.070-0.066-6.687-0.104-10.314-0.104-138.198 0-263.562 54.947-355.438 144.186l0.123-0.119c-98.26 92.852-159.424 224.071-159.424 369.575 0 142.717 58.843 271.691 153.591 363.984l0.111 0.107c88.622 88.958 210.672 144.561 345.709 146.368l0.343 0.004h24.094c277.633-5.364 500.641-231.69 500.641-510.104 0-136.661-53.732-260.772-141.221-352.36l0.185 0.195zM512 894.494v0c-210.453-1.363-380.611-171.944-381.289-382.429l-0-0.065c0.342-210.443 170.847-380.947 381.257-381.289l0.033-0c210.913 0 381.892 170.979 381.892 381.892s-170.979 381.892-381.892 381.892v0z",
            }
        ],
        path![
            attrs!{
                At::D => "M512 234.315c-45.742 0-82.824 37.081-82.824 82.824s37.081 82.824 82.824 82.824c45.742 0 82.824-37.081 82.824-82.824v0c0-45.742-37.081-82.824-82.824-82.824v-0z",
            }
        ],
        path![
            attrs!{
                At::D => "M512 439.115c-0.001-0-0.002-0-0.003-0-39.92 0-72.282 32.362-72.282 72.282 0 0.212 0.001 0.423 0.003 0.635l-0-0.032v225.882c2.528 38.025 33.99 67.91 72.433 67.91s69.905-29.886 72.421-67.691l0.012-0.219v-225.882c0.004-0.269 0.006-0.587 0.006-0.906 0-39.754-32.227-71.981-71.981-71.981-0.214 0-0.427 0.001-0.641 0.003l0.033-0z",
            }
        ],
    ]
}
