use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); })
}

#[view]
pub fn color_picker(color: Option<(&str, &str)>) -> Node<Msg> {
    let value = color.map(|(value, _)| value);
    let title = color.map(|(_, title)| title);
    div![
        C!["option-input-container", "color-input-container", "button-container"],
        s()
            .padding("1.75rem 1rem")
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1 1 50%")
            .flex_direction(CssFlexDirection::Row)
            .position(CssPosition::Relative)
            .z_index("0")
            .cursor(CssCursor::Pointer),
        value.map(|value| {
            s()
                .background_color(value)
        }),
        attrs!{
            At::TabIndex => 0,
            At::Title => title.unwrap_or("Transparent"),
        },
        on_click_not_implemented(),
        IF!(color.is_none() => {
            div![
                C!["transparent-label-container"],
                s()
                    .align_items(CssAlignItems::Center)
                    .border("thin solid hsla(0,0%,100%,0.2)")
                    .bottom("0")
                    .display(CssDisplay::Flex)
                    .justify_content(CssJustifyContent::Center)
                    .left("0")
                    .padding("0 0.5rem")
                    .pointer_events("none")
                    .position(CssPosition::Absolute)
                    .right("0")
                    .top("0")
                    .z_index("0"),
                div![
                    C!["transparent-label"],
                    s()
                        .color(Color::SurfaceLight5)
                        .flex("1")
                        .text_align(CssTextAlign::Center)
                        .text_overflow("ellipsis")
                        .white_space(CssWhiteSpace::NoWrap),
                    "Transparent",
                ]
            ]
        }),
    ]
}
