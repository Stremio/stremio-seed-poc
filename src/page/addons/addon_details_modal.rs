use crate::{PageId, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use std::rc::Rc;
use enclose::enc;
use crate::page::addons::{Msg, format_addon_types};
use stremio_core::types::addon::DescriptorPreview;

#[view]
pub fn modal(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        C!["addon-details-modal-container", "modal-container"],
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
        modal_dialog_container(addon),
    ]
}

#[view]
fn modal_dialog_container(addon: &DescriptorPreview) -> Node<Msg> {
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
        title_container(&addon.manifest.name),
        modal_dialog_content(addon),
        buttons_container(addon),
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
fn title_container(title: &str) -> Node<Msg> {
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
            At::Title => title,
        },
        title,
    ]
}

#[view]
fn modal_dialog_content(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        C!["modal-dialog-content"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .margin("1.5rem 1rem 0")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 1rem"),
        addon_details_container(addon)
    ]
}

#[view]
fn addon_details_container(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        C!["addon-details-container"],
        s()
            .max_width(pc(100))
            .width(rem(40)),
        div![
            C!["title-container"],
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row),
            logo(addon.manifest.logo.as_ref()),
            div![
                C!["name-container"],
                s()
                    .align_items(CssAlignItems::Baseline)
                    .display(CssDisplay::Flex)
                    .flex_basis("0")
                    .flex_direction(CssFlexDirection::Row)
                    .flex_grow("1")
                    .flex_shrink("1")
                    .flex_wrap(CssFlexWrap::Wrap),
                span![
                    C!["name"],
                    s()
                        .color(hsla(0, 0, 0, 0.9))
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .font_size(rem(1.6))
                        .margin_right(rem(0.5)),
                    &addon.manifest.name,
                ],
                span![
                    C!["version"],
                    s()
                        .color(hsla(0, 0, 0, 0.6))
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("1")
                        .flex_shrink("1")
                        .margin_top(rem(0.5)),
                    format!("v. {}", addon.manifest.version.to_string()),
                ]
            ]
        ],
        sections(addon),
    ]
}

#[view]
fn logo(logo: Option<&String>) -> Node<Msg> {
    if let Some(logo) = logo {
        img![
            C!["logo"],
            s()
                .object_fit("contain")
                .object_position("center")
                .background_color(hsl(0, 0, 0))
                .float(CssFloat::Left)
                .height(rem(5))
                .margin_right(rem(1.5))
                .padding(rem(0.5))
                .width(rem(5)),
            attrs!{
                At::Src => logo,
                At::Alt => "",
            }
        ]
    } else {
        svg![
            C!["icon"],
            s()
                .fill(Color::SecondaryVariant1Light3)
                .background_color(hsl(0, 0, 0))
                .float(CssFloat::Left)
                .height(rem(5))
                .margin_right(rem(1.5))
                .padding(rem(0.5))
                .width(rem(5))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::from("icon") => "ic_addons",
                At::ViewBox => "0 0 1043 1024",
            },
            path![
                attrs!{
                    At::D => "M145.468 679.454c-40.056-39.454-80.715-78.908-120.471-118.664-33.431-33.129-33.129-60.235 0-90.353l132.216-129.807c5.693-5.938 12.009-11.201 18.865-15.709l0.411-0.253c23.492-15.059 41.864-7.529 48.188 18.974 0 7.228 2.711 14.758 3.614 22.287 3.801 47.788 37.399 86.785 82.050 98.612l0.773 0.174c10.296 3.123 22.128 4.92 34.381 4.92 36.485 0 69.247-15.94 91.702-41.236l0.11-0.126c24.858-21.654 40.48-53.361 40.48-88.718 0-13.746-2.361-26.941-6.701-39.201l0.254 0.822c-14.354-43.689-53.204-75.339-99.907-78.885l-0.385-0.023c-18.372-2.409-41.562 0-48.188-23.492s11.445-34.635 24.998-47.887q65.054-62.946 130.409-126.795c32.527-31.925 60.235-32.226 90.353 0 40.659 39.153 80.715 78.908 120.471 118.362 8.348 8.594 17.297 16.493 26.82 23.671l0.587 0.424c8.609 7.946 20.158 12.819 32.846 12.819 24.823 0 45.29-18.653 48.148-42.707l0.022-0.229c3.012-13.252 4.518-26.805 8.734-39.755 12.103-42.212 50.358-72.582 95.705-72.582 3.844 0 7.637 0.218 11.368 0.643l-0.456-0.042c54.982 6.832 98.119 49.867 105.048 104.211l0.062 0.598c0.139 1.948 0.218 4.221 0.218 6.512 0 45.084-30.574 83.026-72.118 94.226l-0.683 0.157c-12.348 3.915-25.299 5.722-37.948 8.433-45.779 9.638-60.235 46.984-30.118 82.824 15.265 17.569 30.806 33.587 47.177 48.718l0.409 0.373c31.925 31.925 64.452 62.946 96.075 94.871 13.698 9.715 22.53 25.511 22.53 43.369s-8.832 33.655-22.366 43.259l-0.164 0.111c-45.176 45.176-90.353 90.353-137.035 134.325-5.672 5.996-12.106 11.184-19.169 15.434l-0.408 0.227c-4.663 3.903-10.725 6.273-17.341 6.273-13.891 0-25.341-10.449-26.92-23.915l-0.012-0.127c-2.019-7.447-3.714-16.45-4.742-25.655l-0.077-0.848c-4.119-47.717-38.088-86.476-82.967-97.721l-0.76-0.161c-9.584-2.63-20.589-4.141-31.947-4.141-39.149 0-74.105 17.956-97.080 46.081l-0.178 0.225c-21.801 21.801-35.285 51.918-35.285 85.185 0 1.182 0.017 2.36 0.051 3.533l-0.004-0.172c1.534 53.671 40.587 97.786 91.776 107.115l0.685 0.104c12.649 2.409 25.901 3.313 38.249 6.626 22.588 6.325 30.118 21.685 18.372 41.864-4.976 8.015-10.653 14.937-17.116 21.035l-0.051 0.047c-44.875 44.574-90.353 90.353-135.228 133.12-10.241 14.067-26.653 23.106-45.176 23.106s-34.935-9.039-45.066-22.946l-0.111-0.159c-40.659-38.852-80.414-78.908-120.471-118.362z",
                }
            ]
        ]
    }
}

#[view]
fn sections(addon: &DescriptorPreview) -> Vec<Node<Msg>> {
    nodes![
        addon.manifest.description.as_ref().map(|description| {
            section_description(description)
        }),
        section_url(&addon.transport_url.to_string()),
        section_supported_types(&format_addon_types(&addon.manifest.types)),
    ]
}

#[view]
fn section_description(description: &str) -> Node<Msg> {
    div![
        C!["section-container"],
        s()
            .margin_top(rem(1)),
        span![
            C!["section-label"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1.1))
                .font_weight("300"),
            description,
        ]
    ]
}

#[view]
fn section_url(url: &str) -> Node<Msg> {
    div![
        C!["section-container"],
        s()
            .margin_top(rem(1)),
        span![
            C!["section-header", "transport-url-label"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1.1)),
            "URL: ",
        ],
        span![
            C!["section-label"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1.1))
                .font_weight("300")
                .user_select("text"),
            url,
        ]
    ]
}

#[view]
fn section_supported_types(supported_types: &str) -> Node<Msg> {
    div![
        C!["section-container"],
        s()
            .margin_top(rem(1)),
        span![
            C!["section-header"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1.1)),
            "Supported types: ",
        ],
        span![
            C!["section-label"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .font_size(rem(1.1))
                .font_weight("300"),
            supported_types,
        ]
    ]
}

#[view]
fn buttons_container(addon: &DescriptorPreview) -> Node<Msg> {
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
        uninstall_button(addon),
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
fn uninstall_button(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        C!["uninstall-button", "action-button", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .background_color(Color::Accent2)
            .display(CssDisplay::Flex)
            .flex("1")
            .flex_direction(CssFlexDirection::Row)
            .justify_content(CssJustifyContent::Center)
            .padding(rem(1.2))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::Accent2Light2),
        attrs!{
            At::TabIndex => 0,
            At::Title => "Uninstall",
        },
        ev(Ev::Click, enc!((addon) move |_| Msg::UninstallAddon(addon))),
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .font_size(rem(1.2))
                .font_weight("500")
                .max_height(em(3.5))
                .text_align(CssTextAlign::Center),
            "Uninstall",
        ]
    ]
}
