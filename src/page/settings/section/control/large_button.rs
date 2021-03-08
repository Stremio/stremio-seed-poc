use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn large_button(title: &str, url: impl Into<Option<Url>>) -> Node<Msg> {
    a![
        C!["option-input-container", "button-container"],
        s()
            .background_color(Color::Accent3)
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1 1 50%")
            .flex_direction(CssFlexDirection::Row)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::Accent3Light1),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
        },
        url.into().map(|url| {
            attrs!{
                At::Href => url,
            }
        }),
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
