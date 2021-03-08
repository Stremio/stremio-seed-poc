use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn status() -> Node<Msg> {
    div![
        C!["option-input-container", "info-container"],
        s()
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1 1 50%")
            .flex_direction(CssFlexDirection::Row),
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .line_height(rem(1.5)),
            "Err",
        ]
    ]
}
