use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn label(title: &str) -> Node<Msg> {
    div![
        C!["option-name-container"],
        s()
            .justify_content(CssJustifyContent::FlexStart)
            .margin_right(rem(2))
            .padding("1rem 1rem 1rem 0")
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
            title,
        ]                   
    ]
}
