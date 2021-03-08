use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn dropdown(selected_value: &str) -> Node<Msg> {
    div![
        C!["option-input-container", "multiselect-container", "label-container", "button-container", "disabled"],
        s()
            .padding(rem(1))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1 1 50%")
            .flex_direction(CssFlexDirection::Row)
            .pointer_events("none")
            .background_color(Color::Background)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => -1,
        },
        div![
            C!["label"],
            s()
                .line_height(rem(1.5))
                .max_height(rem(1.5))
                .color(Color::SecondaryVariant1_90)
                .flex("1")
                .font_weight("500"),
            selected_value,
        ],
        svg![
            C!["icon"],
            s()
                .fill(Color::SecondaryVariant1_90)
                .flex(CssFlex::None)
                .height(rem(1))
                .margin_left(rem(1))
                .width(rem(1))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 1024 1024",
                At::from("icon") => "ic_arrow_thin_down",
            },
            path![
                attrs!{
                    At::D => "M14.155 314.428l463.511 465.318c8.928 8.731 21.149 14.127 34.63 14.155l0.005 0c0.103 0.001 0.225 0.001 0.348 0.001 13.437 0 25.582-5.534 34.278-14.448l0.009-0.010 462.908-463.812c8.82-9.052 14.26-21.434 14.26-35.087s-5.44-26.035-14.27-35.098l0.010 0.011c-8.905-8.816-21.115-14.308-34.607-14.456l-0.028-0c-13.572 0.165-25.802 5.779-34.629 14.751l-0.006 0.007-428.574 428.273-427.972-429.779c-8.799-8.927-21.024-14.458-34.541-14.458-0.139 0-0.278 0.001-0.417 0.002l0.021-0c-0.043-0-0.094-0-0.145-0-13.595 0-25.899 5.526-34.789 14.455l-0.002 0.002c-8.82 9.052-14.26 21.434-14.26 35.087s5.44 26.035 14.27 35.098l-0.010-0.011z",
                }
            ],
        ]
    ]
}
