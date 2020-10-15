use seed::{prelude::*, *};
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use crate::styles::{self, themes::{Color, get_color_value}};

// @TODO DRY + add logic + push to url

#[allow(clippy::pub_enum_variant_names)]
#[derive(Clone)]
pub enum Modal {
    AddAddon,
    ShareAddon,
    InstallAddon,
    UninstallAddon,
}

// ------ ------
//    Styles
// ------ ------

fn modal_dialog_container_style() -> Style {
    s()
        .position(CssPosition::Relative)
        .margin(CssMargin::Auto)
        .padding(rem(1))
        .background_color(Color::SurfaceLighter)
}

fn modal_dialog_h1_style() -> Style {
    s()
        .margin_bottom(rem(1))
        .font_size(rem(1.2))
}

fn modal_dialog_content_styles() -> Vec<Style> {
    vec![
        s()
            .padding(rem(1)),
        s()
            .style_other(">:not(:first-child)")
            .margin_top(rem(1)),
    ]
}

fn modal_dialog_buttons_style() -> Style {
    s()
        .margin(rem(1))
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
}

fn modal_dialog_close_button_container_styles() -> Vec<Style> {
    vec![
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .align_items(CssAlignItems::Center)
            .justify_content(CssJustifyContent::Center)
            .position(CssPosition::Absolute)
            .top(rem(0.2))
            .right(rem(0.2))
            .width(rem(2))
            .height(rem(2))
            .padding(rem(0.5)),
        s()
            .hover()
            .background_color(Color::SurfaceLight),
        s()
            .style_other(":hover .icon")
            .fill(Color::Signal2),
        s()
            .style_other(":hover .focus")
            .fill(Color::Signal2),
    ]
}

fn model_dialog_close_button_icon_style() -> Style {
    s()
        .flex(CssFlex::None)
        .display(CssDisplay::Block)
        .width(pc(100))
        .height(pc(100))
        .fill(Color::SurfaceDark)
}

fn action_button_styles() -> Vec<Style> {
    vec![
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .align_items(CssAlignItems::Center)
            .justify_content(CssJustifyContent::Center)
            .flex("1 0 0")
            .padding(rem(1))
            .text_align(CssTextAlign::Center)
            .color(Color::SurfaceLighter)
            .background_color(Color::Signal5),
        s()
            .hover()
            .filter("brightness(1.2)"),
        s()
            .focus()
            .outline_width(format!("calc(1.5 * {})", styles::global::FOCUS_OUTLINE_SIZE).as_str())
            .outline_style(CssOutlineStyle::Solid)
            .outline_color(Color::SurfaceLighter)
            .raw(format!("outline-offset: calc(-2 * {});", styles::global::FOCUS_OUTLINE_SIZE).as_str()),
        s()
            .disabled()
            .color(Color::SurfaceDarker)
            .background_color(Color::SurfaceDark),
        s()
            .not(":last-child")
            .margin_right(rem(2)),
    ]
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(modal: &Modal, close_msg: impl Fn() -> Ms + Copy + 'static) -> Node<Ms> {
    div![
        C!["modals-container"],
        s()
            .width("0")
            .height("0"),
        div![
            C!["modal-container",],
            s()
                .position(CssPosition::Absolute)
                .top("0")
                .right("0")
                .bottom("0")
                .left("0")
                .z_index("1")
                .overflow(CssOverflow::Hidden),
            s()
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Column)
                .background_color(Color::BackgroundDark),
            ev(Ev::Click, move |event| {
                IF!(event.target() == event.current_target() => close_msg())
            }),
            match modal {
                Modal::AddAddon => view_add_addon_modal(close_msg),
                Modal::ShareAddon => view_share_addon_modal(close_msg),
                Modal::InstallAddon | Modal::UninstallAddon => view_install_addon_modal(close_msg),
            }
        ]
    ]
}

#[allow(clippy::too_many_lines)]
fn view_install_addon_modal<Ms: 'static>(close_msg: impl Fn() -> Ms + Copy + 'static) -> Node<Ms> {
    div![
        C![
            "addon-prompt-container",
            "modal-dialog-container"
        ],
        modal_dialog_container_style(),
        s()
            .width(rem(50)),
        div![
            C![
                "close-button-container",
                "button-container",
            ],
            modal_dialog_close_button_container_styles(),
            styles::button_container(),
            attrs!{
                At::TabIndex => 0,
                At::Title => "Close",
            },
            ev(Ev::Click, move |_| close_msg()),
            svg![
                C!["icon",],
                model_dialog_close_button_icon_style(),
                attrs! {
                    At::ViewBox => "0 0 1024 1024",
                    "icon" => "ic_x",
                },
                path![attrs! {
                    At::D => "M632.471 512l366.231-365.026c21.288-15.866 34.926-40.97 34.926-69.261 0-47.572-38.565-86.136-86.136-86.136-28.29 0-53.395 13.638-69.098 34.699l-0.162 0.228-366.231 365.026-365.026-366.231c-14.126-10.54-31.928-16.876-51.21-16.876-47.572 0-86.136 38.565-86.136 86.136 0 19.282 6.335 37.084 17.038 51.438l-0.162-0.228 365.026 366.231-366.231 365.026c-21.288 15.866-34.926 40.97-34.926 69.261 0 47.572 38.565 86.136 86.136 86.136 28.29 0 53.395-13.638 69.098-34.699l0.162-0.228 366.231-365.026 365.026 366.231c15.866 21.288 40.97 34.926 69.261 34.926 47.572 0 86.136-38.565 86.136-86.136 0-28.29-13.638-53.395-34.699-69.098l-0.228-0.162z"
                }]
            ],
        ],
        div![
            C![
                "modal-dialog-content",
            ],
            modal_dialog_content_styles(),
            div![
                div![
                    C![
                        "title-container",
                        "title-with-logo-container"
                    ],
                    s()
                        .font_size(rem(3))
                        .font_weight("300")
                        .word_break("break-all"),
                    s()
                        .first_line()
                        .line_height(rem(5)),
                    s()
                        .text_align(CssTextAlign::Center),
                    div![
                        C![
                            "logo-container"
                        ],
                        s()
                            .width(rem(5))
                            .height(rem(5))
                            .margin_right(rem(0.5))
                            .background_color(Color::SurfaceLight20)
                            .float(CssFloat::Left),
                        img![
                            C![
                                "logo"
                            ],
                            s()
                                .display(CssDisplay::Block)
                                .width(pc(100))
                                .height(pc(100))
                                .raw(r#"object-fit: contain;"#)
                                .raw(r#"object-position: center;"#),
                            attrs!{
                                At::Src => "https://holamovies.herokuapp.com/holamovies.png",
                            }
                        ]
                    ],
                    "¡Hola! Movies ",
                    span![
                        C![
                            "version-container",
                        ],
                        s()
                            .font_size(rem(1.5))
                            .font_weight("400"),
                        "v.2.0.1"
                    ]
                ],
                div![
                    C![
                        "section-container",
                    ],
                    s()
                        .margin_top(rem(1)),
                    span![
                        C![
                            "section-header"
                        ],
                        s()
                            .font_size(rem(1.2)),
                        "Watch movies in spanish and english"
                    ]
                ],
                div![
                    C![
                        "section-container",
                    ],
                    s()
                        .margin_top(rem(1)),
                    span![
                        C![
                            "section-header"
                        ],
                        s()
                            .font_size(rem(1.2)),
                        "URL: ",
                    ],
                    span![
                        C![
                            "addon-prompt-section-label",
                            "transport-url-label"
                        ],
                        s()
                            .font_size(rem(1.2))
                            .font_weight("300"),
                        s()
                            .user_select("text"),
                        "https://holamovies.herokuapp.com/manifest.json",
                    ]
                ],
                div![
                    C![
                        "section-container",
                    ],
                    s()
                        .margin_top(rem(1)),
                    span![
                        C![
                            "section-header"
                        ],
                        s()
                            .font_size(rem(1.2)),
                        "Supported types: ",
                    ],
                    span![
                        C![
                            "addon-prompt-section-label",
                            "transport-url-label"
                        ],
                        s()
                            .font_size(rem(1.2))
                            .font_weight("300"),
                        s()
                            .user_select("text"),
                        "movie",
                    ]
                ],
                div![
                    C![
                        "section-container",
                    ],
                    s()
                        .margin_top(rem(1)),
                    span![
                        C![
                            "section-header"
                        ],
                        s()
                            .font_size(rem(1.2)),
                        "Supported catalogs: ",
                    ],
                    span![
                        C![
                            "addon-prompt-section-label",
                            "transport-url-label"
                        ],
                        s()
                            .font_size(rem(1.2))
                            .font_weight("300"),
                        s()
                            .user_select("text"),
                        "",
                    ]
                ],
                div![
                    C![
                        "section-container",
                    ],
                    s()
                        .margin_top(rem(1)),
                    div![
                        C![
                            "addon-prompt-section-label",
                            "disclaimer-label",
                        ],
                        s()
                            .font_size(rem(1.2))
                            .font_weight("300"),
                        s()
                            .font_style(CssFontStyle::Italic),
                        "Using third-party add-ons will always be subject to your responsibility and the governing law of the jurisdiction you are located.",
                    ]
                ],
            ]
        ],
        div![
            C![
                "modal-dialog-buttons",
            ],
            modal_dialog_buttons_style(),
            div![
                C![
                    "cancel-button",
                    "action-button",
                    "button-container",
                ],
                action_button_styles(),
                styles::button_container(),
                s()
                    .background_color(Color::SurfaceDark),
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Cancel"
                },
                ev(Ev::Click, move |_| close_msg()),
                "Cancel",
            ],
            div![
                C![
                    "action-button",
                    "button-container",
                ],
                action_button_styles(),
                styles::button_container(),
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Install"
                },
                "Install",
            ],
        ],
    ]
}

#[allow(clippy::too_many_lines)]
fn view_share_addon_modal<Ms: 'static>(close_msg: impl Fn() -> Ms + Copy + 'static) -> Node<Ms> {
    let share_buttons_button_container_styles = vec![
        s()
            .flex_grow("0")
            .flex_shrink("1")
            .flex_basis("14rem")
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .align_items(CssAlignItems::Center)
            .justify_content(CssJustifyContent::Center)
            .padding("0.6rem 1rem"),
        s()
            .hover()
            .filter("brightness(1.2)"),
        s()
            .focus()
            .outline_width(format!("calc(1.5 * {})", styles::global::FOCUS_OUTLINE_SIZE).as_str())
            .outline_style(CssOutlineStyle::Solid)
            .outline_color(Color::SurfaceLighter)
            .raw(format!("outline-offset: calc(-2 * {});", styles::global::FOCUS_OUTLINE_SIZE).as_str()),
        s()
            .not(":last-child")
            .margin_right(rem(2)),
    ];

    let share_button_icon_style = 
        s()
            .flex(CssFlex::None)
            .width(rem(1.4))
            .height(rem(1.4))
            .margin_right(rem(0.6))
            .fill(Color::SurfaceLighter);

    let share_button_label_style =
        s()
            .flex_grow("0")
            .flex_shrink("1")
            .flex_basis(CssFlexBasis::Auto)
            .font_size(rem(0.8))
            .font_weight("500")
            .color(Color::SurfaceLighter)
            .text_align(CssTextAlign::Center);
    
    div![
        C!["share-prompt-container", "modal-dialog-container"],
        modal_dialog_container_style(),
        s()
            .width(rem(30)),
        div![
            C!["close-button-container", "button-container",],
            modal_dialog_close_button_container_styles(),
            styles::button_container(),
            attrs! {
                At::TabIndex => 0,
                At::Title => "Close",
            },
            ev(Ev::Click, move |_| close_msg()),
            svg![
                C!["icon",],
                model_dialog_close_button_icon_style(),
                attrs! {
                    At::ViewBox => "0 0 1024 1024",
                    "icon" => "ic_x",
                },
                path![attrs! {
                    At::D => "M632.471 512l366.231-365.026c21.288-15.866 34.926-40.97 34.926-69.261 0-47.572-38.565-86.136-86.136-86.136-28.29 0-53.395 13.638-69.098 34.699l-0.162 0.228-366.231 365.026-365.026-366.231c-14.126-10.54-31.928-16.876-51.21-16.876-47.572 0-86.136 38.565-86.136 86.136 0 19.282 6.335 37.084 17.038 51.438l-0.162-0.228 365.026 366.231-366.231 365.026c-21.288 15.866-34.926 40.97-34.926 69.261 0 47.572 38.565 86.136 86.136 86.136 28.29 0 53.395-13.638 69.098-34.699l0.162-0.228 366.231-365.026 365.026 366.231c15.866 21.288 40.97 34.926 69.261 34.926 47.572 0 86.136-38.565 86.136-86.136 0-28.29-13.638-53.395-34.699-69.098l-0.228-0.162z"
                }]
            ],
        ],
        h1![
            modal_dialog_h1_style(),
            "Share addon"
        ],
        div![
            C!["modal-dialog-content",],
            modal_dialog_content_styles(),
            div![
                div![
                    C!["buttons-container",],
                    s()
                        .flex(CssFlex::None)
                        .align_self(CssAlignSelf::Stretch)
                        .display(CssDisplay::Flex)
                        .flex_direction(CssFlexDirection::Row),
                    a![
                        C!["button-container", "facebook-button",],
                        styles::button_container(),
                        &share_buttons_button_container_styles,
                        s()
                            .background_color(styles::global::COLOR_FACEBOOK),
                        attrs! {
                            At::TabIndex => 0,
                            At::Href => "https://example.com",
                            At::Target => "_blank",
                        },
                        svg![
                            C!["icon",],
                            &share_button_icon_style,
                            attrs! {
                                At::ViewBox => "0 0 474 1024",
                                "icon" => "ic_facebook",
                            },
                            path![attrs! {
                                At::D => "M474.052 331.294h-161.431v-106.014c-0.245-1.731-0.385-3.731-0.385-5.764 0-23.952 19.417-43.369 43.369-43.369 0.665 0 1.326 0.015 1.984 0.045l-0.093-0.003h114.146v-176.188h-156.913c-174.381 0-213.835 131.012-213.835 214.739v116.555h-100.894v180.706h100.894v512h210.824v-512h143.059z"
                            }]
                        ],
                        div![
                            C!["label"], 
                            &share_button_label_style,
                            "FACEBOOK",
                        ]
                    ],
                    a![
                        C!["button-container", "twitter-button",],
                        styles::button_container(),
                        &share_buttons_button_container_styles,
                        s()
                            .background_color(styles::global::COLOR_TWITTER),
                        attrs! {
                            At::TabIndex => 0,
                            At::Href => "https://example.com",
                            At::Target => "_blank",
                        },
                        svg![
                            C!["icon",],
                            &share_button_icon_style,
                            attrs! {
                                At::ViewBox => "0 0 1254 1024",
                                "icon" => "ic_twitter",
                            },
                            path![attrs! {
                                At::D => "M0 905.939c17.259 2.039 37.252 3.203 57.516 3.203 119.068 0 228.753-40.171 316.243-107.7l-1.203 0.893c-22.743-1.698-43.86-5.98-63.924-12.583l1.881 0.536c-70.906-20.856-127.859-69.551-159.254-133.124l-0.671-1.502c-4.071-7.73-7.946-16.895-11.073-26.401l-0.372-1.307c-2.108-6.325 0-8.734 6.626-7.529 11.971 1.873 25.777 2.943 39.832 2.943 17.867 0 35.331-1.729 52.233-5.028l-1.712 0.278 10.24-2.711c-34.455-7.267-64.815-21.216-90.995-40.508l0.642 0.451c-60.815-42.177-101.97-109.023-108.953-185.742l-0.073-0.987c0-6.927 0-13.854 0-20.48s2.409-9.638 9.035-6.024c12.154 6.37 26.429 12.204 41.285 16.703l1.783 0.464c16.636 5.729 35.806 9.036 55.749 9.036 0.201 0 0.402-0 0.602-0.001l-0.031 0c-10.719-9.567-20.551-19.399-29.803-29.759l-0.314-0.358c-41.391-40.985-68.762-96.061-74.319-157.434l-0.072-0.985c-0.599-6.701-0.94-14.495-0.94-22.37 0-35.322 6.871-69.040 19.349-99.889l-0.639 1.788c3.313-9.035 8.433-17.167 12.348-25.901s6.024-6.626 10.541-1.506c9.035 10.541 18.673 21.082 28.009 30.118 41.819 45.8 89.007 85.615 140.853 118.79l2.808 1.681c67.118 43.812 145.333 77.707 229.074 97.214l4.94 0.969c30.899 6.653 68.49 12.096 106.754 15.156l3.176 0.204c6.325 0 7.831-1.807 6.927-7.529-1.807-12.047-3.915-23.793-4.518-35.84-0.273-4.4-0.429-9.544-0.429-14.723 0-65.781 25.106-125.699 66.259-170.693l-0.174 0.193c47.406-53.456 116.263-86.977 192.949-86.977 68.273 0 130.34 26.57 176.426 69.933l-0.132-0.123c4.884 5.577 12.018 9.079 19.971 9.079 2.872 0 5.637-0.457 8.226-1.301l-0.187 0.053c52.579-13.389 98.691-32.168 141.384-56.292l-2.843 1.477c2.711-1.506 5.421-4.819 8.433-2.409s0 5.722 0 8.433c-20.092 53.257-54.499 97.274-98.544 128.634l-0.844 0.571c10.842-1.807 22.287-3.012 33.129-5.421s23.492-5.421 34.936-8.734 22.287-6.626 33.431-10.541 21.384-8.433 33.431-11.746c-5.043 11.548-11.411 21.487-19.088 30.25l0.114-0.132c-28.487 36.762-60.588 68.671-96.333 96.061l-1.248 0.918c-5.741 3.761-9.48 10.164-9.48 17.44 0 0.859 0.052 1.706 0.153 2.538l-0.010-0.1c0.226 6.664 0.354 14.497 0.354 22.36 0 39.211-3.195 77.676-9.34 115.148l0.553-4.087c-11.548 67.92-30.080 128.603-55.27 185.688l1.961-4.982c-27.51 62.763-59.97 116.862-98.284 166.189l1.305-1.747c-41.023 52.791-87.62 98.431-139.649 137.185l-1.904 1.356c-76.627 56.553-167.686 98.485-266.451 119.644l-4.607 0.827c-47.911 10.463-102.945 16.456-159.38 16.456-18.101 0-36.057-0.617-53.85-1.83l2.406 0.131c-70.124-4.425-135.583-18.303-197.118-40.41l4.968 1.558c-57.442-19.942-106.979-43.852-153.173-72.626l3.187 1.85c-2.6-1.080-4.467-3.449-4.815-6.287l-0.004-0.038z"
                            }]
                        ],
                        div![
                            C!["label"],
                            &share_button_label_style,
                            "TWITTER",
                        ]
                    ],
                ],
                div![
                    C!["url-container"],
                    s()
                        .display(CssDisplay::Flex)
                        .flex_direction(CssFlexDirection::Row)
                        .margin_top(rem(2))
                        .border(format!("thin solid {}", get_color_value(Color::Surface)).as_str()),
                    input![
                        C!["url-content", "text-input",],
                        styles::text_input(),
                        s()
                            .flex("1")
                            .min_width(rem(12))
                            .padding("0.6rem 1rem")
                            .font_size(rem(0.9))
                            .color(Color::SurfaceDark)
                            .text_align(CssTextAlign::Center)
                            .border_right(format!("thin solid {}", get_color_value(Color::Surface)).as_str()),
                        attrs! {
                            At::Size => 1,
                            // @TODO typed names once Seed has all official types attrs
                            // @TODO (https://github.com/seed-rs/seed/issues/261#issuecomment-555138892)
                            "autocorrect" => "off",
                            "autocapitalize" => "off",
                            At::AutoComplete => "off",
                            At::SpellCheck => "false",
                            At::TabIndex => -1,
                            At::Type => "text",
                            At::Value => "https://example.com",
                            At::ReadOnly => true.as_at_value(),
                        }
                    ],
                    div![
                        C!["copy-button", "button-container",],
                        styles::button_container(),
                        s()
                            .flex_grow("0")
                            .flex_shrink("1")
                            .flex_basis(CssFlexBasis::Auto)
                            .display(CssDisplay::Flex)
                            .flex_direction(CssFlexDirection::Row)
                            .align_items(CssAlignItems::Center)
                            .justify_content(CssJustifyContent::Center)
                            .padding("0.6rem 1rem")
                            .background_color(Color::Surface),
                        s()
                            .hover()
                            .filter("brightness(1.2)"),
                        s()
                            .focus()
                            .outline_width(format!("calc(1.5 * {})", styles::global::FOCUS_OUTLINE_SIZE).as_str())
                            .outline_style(CssOutlineStyle::Solid)
                            .outline_color(Color::SurfaceLighter)
                            .raw(format!("outline-offset: calc(-1.5 * {});", styles::global::FOCUS_OUTLINE_SIZE).as_str()),
                        attrs! {
                            At::TabIndex => 0,
                        },
                        svg![
                            C!["icon",],
                            s()
                                .flex(CssFlex::None)
                                .width(rem(1.4))
                                .height(rem(1.4))
                                .margin_right(rem(0.6))
                                .fill(Color::SurfaceDarker),
                            attrs! {
                                At::ViewBox => "0 0 1048 1024",
                                "icon" => "ic_link",
                            },
                            path![attrs! {
                                At::D => "M1030.325 148.179c-36.472-87.691-121.454-148.225-220.574-148.225-1.655 0-3.306 0.017-4.952 0.050l0.246-0.004c-3.361-0.181-7.295-0.284-11.254-0.284-54.84 0-105.054 19.807-143.878 52.658l0.327-0.27c-30.118 24.998-57.525 53.609-85.835 80.715s-60.235 60.235-90.353 90.353c-9.338 8.314-15.191 20.369-15.191 33.79 0 6.514 1.379 12.706 3.86 18.3l-0.114-0.288c5.862 15.985 20.431 27.416 37.814 28.604l0.134 0.007c0.505 0.016 1.098 0.025 1.694 0.025 17.862 0 33.812-8.184 44.306-21.006l0.081-0.102 146.071-145.769c28.015-28.297 66.867-45.818 109.813-45.818 41.281 0 78.781 16.19 106.496 42.566l-0.064-0.061c26.436 26.57 42.779 63.205 42.779 103.657 0 42.223-17.805 80.288-46.316 107.096l-0.076 0.071c-68.367 67.464-137.035 134.325-205.704 201.788-10.403 10.47-22.177 19.559-35.035 26.979l-0.805 0.428c-21.108 11.626-46.265 18.466-73.019 18.466-48.785 0-92.258-22.743-120.394-58.205l-0.243-0.317c-19.576-24.094-38.551-30.118-60.235-21.685-32.226 13.252-37.647 46.682-12.649 78.607 45.802 57.384 115.741 93.831 194.194 93.831 66.707 0 127.259-26.351 171.816-69.212l-0.080 0.077q107.219-103.002 212.932-206.908c44.223-43.333 71.637-103.676 71.637-170.421 0-32.255-6.402-63.015-18.005-91.078l0.58 1.584zM504.169 739.991c-49.995 48.188-99.388 97.28-149.082 145.769-28.049 28.832-67.221 46.72-110.571 46.72-40.934 0-78.143-15.95-105.75-41.973l0.077 0.072c-27.279-26.705-44.192-63.906-44.192-105.058 0-42.901 18.381-81.508 47.697-108.377l0.109-0.099c67.765-66.861 136.132-133.421 204.198-199.981 27.819-29.22 67.011-47.393 110.448-47.393 48.209 0 91.189 22.385 119.114 57.328l0.235 0.305c8.488 12.105 22.378 19.922 38.093 19.922 25.616 0 46.381-20.766 46.381-46.381 0-12.256-4.753-23.401-12.517-31.693l0.024 0.026c-45.804-55.809-114.788-91.135-192.024-91.135-64.703 0-123.616 24.792-167.757 65.394l0.176-0.16c-74.692 70.776-147.576 143.059-220.461 215.944-42.105 42.532-68.119 101.063-68.119 165.67 0 33.514 7 65.393 19.618 94.255l-0.591-1.516c36.633 86.692 120.955 146.421 219.227 146.421 1.705 0 3.406-0.018 5.102-0.054l-0.253 0.004c3.299 0.175 7.162 0.275 11.048 0.275 57.5 0 109.9-21.834 149.362-57.663l-0.184 0.165c60.235-53.308 114.447-110.231 170.767-165.948 8.849-7.922 14.392-19.38 14.392-32.133 0-7.182-1.758-13.953-4.867-19.907l0.113 0.238c-7.634-16.234-23.852-27.272-42.649-27.272-15.072 0-28.487 7.097-37.084 18.132l-0.079 0.105z"
                            }]
                        ],
                        div![
                            C!["label"], 
                            s()
                                .color(Color::SurfaceDarker),
                            "Copy"
                        ]
                    ]
                ]
            ]
        ],
    ]
}

fn view_add_addon_modal<Ms: 'static>(close_msg: impl Fn() -> Ms + Copy + 'static) -> Node<Ms> {
    div![
        C!["add-addon-prompt-container", "modal-dialog-container"],
        modal_dialog_container_style(),
        s()
            .width(rem(30)),
        div![
            C!["close-button-container", "button-container",],
            modal_dialog_close_button_container_styles(),
            styles::button_container(),
            attrs! {
                At::TabIndex => 0,
                At::Title => "Close",
            },
            ev(Ev::Click, move |_| close_msg()),
            svg![
                C!["icon",],
                model_dialog_close_button_icon_style(),
                attrs! {
                    At::ViewBox => "0 0 1024 1024",
                    "icon" => "ic_x",
                },
                path![attrs! {
                    At::D => "M632.471 512l366.231-365.026c21.288-15.866 34.926-40.97 34.926-69.261 0-47.572-38.565-86.136-86.136-86.136-28.29 0-53.395 13.638-69.098 34.699l-0.162 0.228-366.231 365.026-365.026-366.231c-14.126-10.54-31.928-16.876-51.21-16.876-47.572 0-86.136 38.565-86.136 86.136 0 19.282 6.335 37.084 17.038 51.438l-0.162-0.228 365.026 366.231-366.231 365.026c-21.288 15.866-34.926 40.97-34.926 69.261 0 47.572 38.565 86.136 86.136 86.136 28.29 0 53.395-13.638 69.098-34.699l0.162-0.228 366.231-365.026 365.026 366.231c15.866 21.288 40.97 34.926 69.261 34.926 47.572 0 86.136-38.565 86.136-86.136 0-28.29-13.638-53.395-34.699-69.098l-0.228-0.162z"
                }]
            ],
        ],
        h1![
            modal_dialog_h1_style(),
            "Add addon"
        ],
        div![
            C!["modal-dialog-content",],
            modal_dialog_content_styles(),
            input![
                C!["url-content", "text-input",],
                styles::text_input(),
                s()
                    .flex("1")
                    .width(pc(100))
                    .padding(rem(0.5))
                    .font_size(rem(0.9))
                    .color(Color::SurfaceDark)
                    .border(format!("thin solid {}", get_color_value(Color::Surface)).as_str()),
                attrs! {
                    At::Size => 1,
                    // @TODO typed names once Seed has all official types attrs
                    // @TODO (https://github.com/seed-rs/seed/issues/261#issuecomment-555138892)
                    "autocorrect" => "off",
                    "autocapitalize" => "off",
                    At::AutoComplete => "off",
                    At::SpellCheck => "false",
                    At::TabIndex => -1,
                    At::Type => "text",
                    At::Placeholder => "Paste url...",
                }
            ]
        ],
        div![
            C!["modal-dialog-buttons",],
            modal_dialog_buttons_style(),
            div![
                C!["cancel-button", "action-button", "button-container",],
                action_button_styles(),
                styles::button_container(),
                s()
                    .background_color(Color::SurfaceDark),
                attrs! {
                    At::TabIndex => 0,
                    At::Title => "Cancel",
                },
                ev(Ev::Click, move |_| close_msg()),
                "Cancel",
            ],
            div![
                C!["action-button", "button-container",],
                action_button_styles(),
                styles::button_container(),
                attrs! {
                    At::TabIndex => 0,
                    At::Title => "Add",
                },
                "Add",
            ],
        ],
    ]
}
