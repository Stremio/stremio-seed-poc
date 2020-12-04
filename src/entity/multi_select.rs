use itertools::Itertools;
use seed::{prelude::*, *};
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use seed_styles::{pc, em, rem};
use seed_styles::*;
use crate::styles::{self, themes::{Color, get_color_value}};

const MENU_CLASS: &str = "popup-menu-container";

pub struct Item<Ms> {
    pub title: String,
    pub selected: bool,
    pub on_click: Rc<dyn Fn() -> Ms>,
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(title: &str, items: Vec<Item<Ms>>) -> Node<Ms> {
    let active = true;
    let selected_item = items.iter().find(|item| item.selected);
    let left_margin = true;
    div![
        C!["select-input", "label-container", "button-container", IF!(active => "active")],
        s()
            .flex("0 1 15rem")
            .height(rem(3.5))
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .padding("0 1rem")
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .cursor(CssCursor::Pointer),
        IF!(left_margin => s().margin_left(rem(1.5))),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
        },
        div![
            C!["label", IF!(active => "active")],
            s()
                .color(if active { Color::SurfaceLight5_90 } else { Color::SecondaryVariant1_90 })
                .flex("1")
                .font_weight("500")
                .max_height(rem(2.4)),
            selected_item.map(|item| &item.title),
        ],
        svg![
            C!["icon"],
            s()
                .fill(if active { Color::SurfaceLight5_90 } else { Color::SecondaryVariant1_90 })
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
            ]
        ],
        IF!(active => menu(items)),
    ]
}

fn menu<Ms: 'static>(items: Vec<Item<Ms>>) -> Node<Ms> {
    div![
        C!["menu-container", "menu-direction-bottom-right"],
        s()
            .bottom("initial")
            .left("0")
            .right("initial")
            .top("100%")
            .visibility(CssVisibility::Visible)
            .width(pc(100))
            .box_shadow(format!(
                "0 1.35rem 2.7rem {}, 0 1.1rem 0.85rem {}",
                "hsla(0,0%,0%,0.4)",
                "hsla(0,0%,0%,0.2)",
            ).as_str())
            .cursor(CssCursor::Auto)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Absolute)
            .z_index("1"),
        div![
            C!["menu-container"],
            s()
                .max_height("calc(3.2rem * 7)")
                .overflow(CssOverflow::Auto),
            items.into_iter().map(menu_item),
        ]
    ]
}

fn menu_item<Ms: 'static>(item: Item<Ms>) -> Node<Ms> {
    div![
        C!["option-container", "button-container", IF!(item.selected => "selected")],
        s()
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .padding(rem(1))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => 0,
            At::Title => item.title,
        },
        {
            let on_click = item.on_click;
            ev(Ev::Click, move |_| on_click())
        },
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex("1")
                .max_height(em(4.8)),
            item.title,
        ],
        IF!(item.selected => {
            div![
                C!["icon"],
                s()
                    .display(CssDisplay::Block)
                    .background_color(Color::Accent3_90)
                    .border_radius(pc(100))
                    .flex(CssFlex::None)
                    .height(rem(0.5))
                    .margin_left(rem(1))
                    .width(rem(0.5))
            ]
        })
    ]
}
