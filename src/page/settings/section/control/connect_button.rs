use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn connect_button(
    title: &str, 
    icon: &str, 
    view_box: &str, 
    on_click: impl Into<Option<EventHandler<Msg>>>,
    enabled: bool,
    paths: Vec<Node<Msg>>,
) -> Node<Msg> {
    div![
        C!["option-input-container", "button-container", IF!(not(enabled) => "disabled")],
        s()
            .background_color(Color::Accent3)
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
        s()
            .hover()
            .background_color(Color::Accent3Light1),
        attrs!{
            At::TabIndex => if enabled { 0 } else { -1 },
            At::Title => title,
        },
        on_click.into(),
        svg![
            C!["icon"],
            s()
                .fill(Color::SurfaceLight5_90)
                .flex(CssFlex::None)
                .height(rem(1.5))
                .margin_right(rem(0.5))
                .width(rem(1.5))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => view_box,
                At::from("icon") => icon,
            },
            paths,
        ],
        div![
            C!["label"],
            s()
                .font_weight("500")
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .line_height(rem(1.5)),
            title,
        ],
    ]
}
