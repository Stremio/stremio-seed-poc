use crate::{PageId, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use std::rc::Rc;
use crate::page::addons::Msg;

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); })
}

#[view]
pub fn modal() -> Node<Msg> {
    div![
        C!["add-addon-modal-container", "modal-container"],
        s()
            .bottom("0")
            .left("0")
            .overflow(CssOverflow::Hidden)
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("1")
            .align_items(CssAlignItems::Center)
            .background_color(hsla(0, 0, 0, 0.4))
            .display(CssDisplay::Flex)
            .justify_content(CssJustifyContent::Center),
        ev(Ev::Click, |_| Msg::CloseModal),
        modal_dialog_container(),
    ]
}

#[view]
fn modal_dialog_container() -> Node<Msg> {
    div![
        C!["modal-dialog-container"],
        s()
            .background_color(Color::SurfaceLight5)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Column)
            .max_height(pc(80))
            .max_width(pc(80)),
        close_button_container(),
        title_container(),
        modal_dialog_content(),
        buttons_container(),
        ev(Ev::Click, |event| event.stop_propagation()),
    ]
}

#[view]
fn close_button_container() -> Node<Msg> {
    div![
        C!["close-button-container", "button-container"],
        s()
            .align_self(CssAlignSelf::FlexEnd)
            .flex(CssFlex::None)
            .height(rem(2))
            .margin("0.2rem 0.2rem 0 0")
            .padding(rem(0.5))
            .width(rem(2))
            .cursor(CssCursor::Pointer),
        s()
            .style_other(":hover .icon")
            .fill(Color::SurfaceLight1_90),
        attrs!{
            At::TabIndex => 0,
            At::Title => "Close",
        },
        ev(Ev::Click, |_| Msg::CloseModal),
        svg![
            C!["icon"],
            s()
                .display(CssDisplay::Block)
                .fill(Color::SurfaceDark1_90)
                .height(pc(100))
                .width(pc(100))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 1024 1024",
                At::from("icon") => "ic_x",
            },
            path!{
                attrs!{
                    At::D => "M632.471 512l366.231-365.026c21.288-15.866 34.926-40.97 34.926-69.261 0-47.572-38.565-86.136-86.136-86.136-28.29 0-53.395 13.638-69.098 34.699l-0.162 0.228-366.231 365.026-365.026-366.231c-14.126-10.54-31.928-16.876-51.21-16.876-47.572 0-86.136 38.565-86.136 86.136 0 19.282 6.335 37.084 17.038 51.438l-0.162-0.228 365.026 366.231-366.231 365.026c-21.288 15.866-34.926 40.97-34.926 69.261 0 47.572 38.565 86.136 86.136 86.136 28.29 0 53.395-13.638 69.098-34.699l0.162-0.228 366.231-365.026 365.026 366.231c15.866 21.288 40.97 34.926 69.261 34.926 47.572 0 86.136-38.565 86.136-86.136 0-28.29-13.638-53.395-34.699-69.098l-0.228-0.162z",
                }
            }
        ]
    ]
}

#[view]
fn title_container() -> Node<Msg> {
    div![
        C!["title-container"],
        s()
            .color(hsla(0, 0, 0, 0.9))
            .flex("1 0 auto")
            .font_size(rem(1.2))
            .font_weight("500")
            .margin("0 2rem")
            .max_height(em(2.4)),
        attrs!{
            At::Title => "Add addon",
        },
        "Add addon",
    ]
}

#[view]
fn modal_dialog_content() -> Node<Msg> {
    div![
        C!["modal-dialog-content"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .margin("1.5rem 1rem 0")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 1rem")
            .width(rem(30)),
        div![
            C!["notice"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1))
                .margin_bottom(rem(1.5)),
            "You can add an addon via an external link, which will appear under Installed addons.",
        ],
        input![
            C!["addon-url-input", "text-input"],
            s()
                .color(Color::SurfaceDark5)
                .padding(rem(1))
                .width(pc(100))
                .outline_width(global::FOCUS_OUTLINE_SIZE)
                .outline_color(Color::SurfaceLight2)
                .outline_style(CssOutlineStyle::Solid)
                .raw(format!("outline-offset: calc(-1 * {});", global::FOCUS_OUTLINE_SIZE).as_str()),
            s()
                .hover()
                .outline_color(Color::SurfaceLight4),
            s()
                .focus()
                .outline_color(hsl(0, 0, 0)),
            attrs!{
                At::Size => 1,
                At::from("autocorrect") => "off",
                At::from("autocapitalize") => "none",
                At::AutoComplete => "off",
                At::from("spellcheck") => false,
                At::TabIndex => 0,
                At::Type => "text",
                At::Placeholder => "Paste addon URL",
            }
        ]
    ]
}

#[view]
fn buttons_container() -> Node<Msg> {
    div![
        C!["buttons-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .margin("2rem 2rem"),
        cancel_button(),
        add_button(),
    ]
}

#[view]
fn cancel_button() -> Node<Msg> {
    div![
        C!["cancel-button", "action-button", "button-container"],
        s()
            .background_color(Color::Transparent)
            .margin_right(rem(1))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex("1")
            .flex_direction(CssFlexDirection::Row)
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1.2))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::SurfaceLight3),
        attrs!{
            At::TabIndex => 0,
            At::Title => "Cancel",
        },
        ev(Ev::Click, |_| Msg::CloseModal),
        div![
            C!["label"],
            s()
                .color(Color::SurfaceDark2)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .font_size(rem(1.1))
                .font_weight("500")
                .max_height(em(3.5))
                .text_align(CssTextAlign::Center),
            "Cancel",
        ]
    ]
}

#[view]
fn add_button() -> Node<Msg> {
    div![
        C!["action-button", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .background_color(Color::Accent3)
            .display(CssDisplay::Flex)
            .flex("1")
            .flex_direction(CssFlexDirection::Row)
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1.2))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::Accent3Light1),
        attrs!{
            At::TabIndex => 0,
            At::Title => "Add",
        },
        on_click_not_implemented(),
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .font_size(rem(1.1))
                .font_weight("500")
                .max_height(em(3.5))
                .text_align(CssTextAlign::Center),
            "Add",
        ]
    ]
}
