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
pub fn nav_bar(title: &str, fullscreen: bool) -> Node<Msg> {
    nav![
        C!["layer", "nav-bar-layer", "horizontal-nav-bar-container"],
        s()
            .background_color("transparent")
            .bottom("initial")
            .overflow(CssOverflow::Visible)
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0")
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .padding_right(rem(1)),
        s()
            .before()
            .box_shadow("0 0 8rem 6rem hsl(0deg 0% 0%)")
            .content(r#""""#)
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("-1"),
        back_button(),
        nav_title(title),
        spacing(),
        fullscreen_button(fullscreen),
    ]
}

#[view]
pub fn back_button() -> Node<Msg> {
    div![
        C!["button-container", "back-button-container"],
        s()
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .width(global::VERTICAL_NAV_BAR_SIZE)
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => -1,
        },
        ev(Ev::Click, |_| Url::go_back(1)),
        svg![
            C!["icon"],
            s()
                .fill(hsl(0, 0, 100))
                .flex(CssFlex::None)
                .height(rem(1.7))
                .width(rem(1.7))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 607 1024",
                At::from("icon") => "ic_back_ios",
            },
            path![
                attrs!{
                    At::D => "M607.473 926.419l-412.009-414.419 412.009-414.419-97.28-97.581-510.193 512 510.193 512z",
                }
            ]
        ]
    ]
}

#[view]
pub fn nav_title(title: &str) -> Node<Msg> {
    h2![
        C!["title"],
        s()
            .color(hsl(0, 0, 100))
            .flex("4 0 0")
            .font_size(rem(1.2))
            .font_style(CssFontStyle::Normal)
            .font_weight("500")
            .letter_spacing(rem(0.01))
            .padding("0 1rem")
            .text_overflow("ellipsis")
            .white_space(CssWhiteSpace::NoWrap),
        title,
    ]
}

#[view]
pub fn spacing() -> Node<Msg> {
    div![
        C!["spacing"],
        s()
            .flex("1 0 0")
    ]
}

#[view]
pub fn fullscreen_button(fullscreen: bool) -> Node<Msg> {
    div![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => -1,
            At::Title => if fullscreen { "Exit Fullscreen" } else { "Enter Fullscreen" },
        },
        ev(Ev::Click, |_| Msg::ToggleFullscreen),
        fullscreen_icon(),
    ]
}

#[view]
fn fullscreen_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(hsl(0, 0, 100))
            .flex(CssFlex::None)
            .height(rem(1.7))
            .width(rem(1.7)),
        attrs!{
            At::ViewBox => "0 0 1016 1024",
            At::from("icon") => "ic_fullscreen",
        },
        path![
            attrs!{
                At::D => "M379.784 1.506l-316.235-1.506c-17.58 0.003-33.524 7.011-45.19 18.385l0.014-0.013c-11.345 11.55-18.354 27.393-18.372 44.872l-0 0.003 1.506 316.838c0.663 34.993 28.856 63.187 63.787 63.848l0.063 0.001c0.090 0 0.196 0.001 0.302 0.001 34.492 0 62.473-27.876 62.644-62.328l0-0.016v-253.591h252.386c0.271 0.004 0.59 0.007 0.91 0.007 34.598 0 62.645-28.047 62.645-62.645 0-0.32-0.002-0.639-0.007-0.958l0.001 0.048c-1.004-34.88-29.443-62.792-64.437-62.946l-0.015-0z",
            }
        ],
        path![
            attrs!{
                At::D => "M633.976 128.904h254.494v252.386c-0.004 0.269-0.007 0.586-0.007 0.904 0 34.598 28.047 62.645 62.645 62.645 0.002 0 0.005-0 0.007-0l-0 0c35.122-0.497 63.483-28.753 64.15-63.787l0.001-0.063v-316.838c0.019-0.581 0.030-1.264 0.030-1.95 0-16.946-6.54-32.364-17.233-43.869l0.037 0.040c-11.448-11.329-27.189-18.338-44.568-18.372l-0.007-0-317.139 1.506c-35.189 0.334-63.646 28.686-64.15 63.802l-0.001 0.048c-0.004 0.271-0.007 0.59-0.007 0.91 0 34.282 27.538 62.133 61.7 62.638l0.048 0.001z",
            }
        ],
        path![
            attrs!{
                At::D => "M380.386 895.096h-252.386v-252.386c0.005-0.282 0.007-0.616 0.007-0.95 0-33.753-26.694-61.271-60.122-62.595l-0.12-0.004c-0.448-0.011-0.976-0.018-1.506-0.018-35.762 0-64.753 28.991-64.753 64.753 0 0.006 0 0.012 0 0.018l-0-0.001-1.506 316.838c-0.002 0.18-0.003 0.392-0.003 0.605 0 34.387 27.706 62.303 62.013 62.642l0.032 0h317.139c35.189-0.334 63.646-28.686 64.15-63.802l0.001-0.048c-0.142-35.138-27.992-63.725-62.825-65.050l-0.121-0.004z",
            }
        ],
        path![
            attrs!{
                At::D => "M950.814 580.066c-0.002-0-0.004-0-0.007-0-34.598 0-62.645 28.047-62.645 62.645 0 0.318 0.002 0.635 0.007 0.951l-0.001-0.048v252.386h-252.687c-0.18-0.002-0.392-0.003-0.605-0.003-34.387 0-62.303 27.706-62.642 62.013l-0 0.032c-0.007 0.359-0.011 0.783-0.011 1.207 0 35.554 28.655 64.416 64.13 64.75l0.032 0h316.536c17.385-0.034 33.126-7.043 44.58-18.377l-0.005 0.005c11.345-11.55 18.354-27.393 18.372-44.872l0-0.003v-316.838c-0.677-35.406-29.538-63.849-65.043-63.849-0.004 0-0.008 0-0.012 0l0.001-0z",
            }
        ],
    ]
}
