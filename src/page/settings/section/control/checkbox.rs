use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn checkbox(checked: bool, on_click: impl Into<Option<EventHandler<Msg>>>, enabled: bool) -> Node<Msg> {
    div![
        C!["option-input-container", "checkbox-container", "button-container", IF!(not(enabled) => "disabled")],
        s()
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1 1 50%")
            .flex_direction(CssFlexDirection::Row)
            .cursor(CssCursor::Pointer),
        IF!(not(enabled) => {
            s()
                .pointer_events("none")
        }),
        attrs!{
            At::TabIndex => if enabled { 0 } else { -1 },
        },
        on_click.into(),
        if checked {
            icon_checked()
        } else {
            icon_unchecked()
        }
    ]
}

#[view]
fn icon_checked() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .height(rem(1.5))
            .width(rem(1.5))
            .display(CssDisplay::Block)
            .background_color(Color::PrimaryVariant1)
            .fill(Color::SurfaceLight5)
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 100 100",
        },
        svg![
            attrs!{
                At::ViewBox => "0 0 1331 1024",
                At::X => 10,
                At::Y => 10,
                At::Width => 80,
                At::Height => 80,
                At::from("icon") => "ic_check",
            },
            path![
                attrs!{
                    At::D => "M545.129 1024c-40.334-0.026-76.847-16.363-103.306-42.769l-398.755-397.551c-24.752-26.158-39.97-61.56-39.97-100.516 0-80.839 65.533-146.372 146.372-146.372 38.806 0 74.085 15.101 100.281 39.748l-0.075-0.070 288.226 286.118 536.395-612.593c27.002-30.81 66.432-50.158 110.381-50.158 80.929 0 146.535 65.606 146.535 146.535 0 36.98-13.698 70.761-36.298 96.544l0.144-0.168-639.699 731.256c-25.909 29.451-63.15 48.401-104.838 49.987l-0.272 0.008z",   
                }
            ],
        ],
    ]
}

#[view]
fn icon_unchecked() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .height(rem(1.5))
            .width(rem(1.5))
            .display(CssDisplay::Block)
            .fill(Color::SurfaceLight5)
            .overflow(CssOverflow::Visible),
        attrs!{
            At::ViewBox => "0 0 1024 1024",
            At::from("icon") => "ic_box_empty",
        },
        path![
            attrs!{
                At::D => "M843.294 180.706v662.588h-662.588v-662.588h662.588zM1024 0h-1024v1024h1024v-1024z",   
            }
        ],
    ]
}
