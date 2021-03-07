use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn button_label(title: &str, on_click: impl Into<Option<EventHandler<Msg>>>, enabled: bool) -> Node<Msg> {
    div![
        C!["option-input-container", "link-container", "button-container", IF!(not(enabled) => "disabled")],
        s()
            .flex("0 1 auto")
            .padding("1rem 0")
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .cursor(CssCursor::Pointer),
        s()
            .style_other(":hover > .label")
            .text_decoration(CssTextDecoration::Underline),
        IF!(not(enabled) => {
            s()
                .pointer_events("none")
        }),
        attrs!{
            At::TabIndex => if enabled { 0 } else { -1 },
            At::Title => title,
        }, 
        on_click.into(),
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .line_height(rem(1.5)),
            title,
        ]   
    ]
}
