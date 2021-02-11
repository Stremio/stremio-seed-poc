use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    context: &mut Context,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();

    let model = model.get_or_insert_with(move || Model {
        active_side_menu_button: SideMenuButton::General
    });
    Some(PageId::Settings)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    active_side_menu_button: SideMenuButton 
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SideMenuButton {
    General,
    Player,
    StreamingServer,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    basic_layout(BasicLayoutArgs {
        page_content: settings_content(model, context).map_msg(msg_mapper),
        container_class: "settings-container",
        context,
        page_id,
        search_args: None,
    })
}

#[view]
fn settings_content<'a>(model: &Model, context: &Context) -> Node<Msg> {
    div![
        C!["settings-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(pc(100))
            .width(pc(100)),            
        side_menu_container(model.active_side_menu_button),
        sections_container(&context.root_base_url),
    ]
}

#[view]
fn side_menu_container(active_button: SideMenuButton) -> Node<Msg> {
    let app_version = "5.0.0";
    div![
        C!["side-menu-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Column)
            .padding(rem(3))
            .width(rem(20)),
        side_menu_button("General", active_button == SideMenuButton::General),
        side_menu_button("Player", active_button == SideMenuButton::Player),
        side_menu_button("Streaming server", active_button == SideMenuButton::StreamingServer),
        div![
            C!["spacing"],
            s()
                .flex("1"),
        ],
        div![
            C!["version-info-label"],
            s()
                .color(Color::SecondaryVariant1_90)
                .flex("0 1 auto")
                .margin("0.5rem 0"),
            attrs!{
                At::Title => app_version,
            },
            "App Version: ",
            app_version,
        ]
    ]
}

#[view]
fn side_menu_button(title: &str, active: bool) -> Node<Msg> {
    div![
        C!["side-menu-button", IF!(active => "selected"), "button-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .color(Color::SecondaryVariant1_90)
            .flex(CssFlex::None)
            .font_size(rem(1.1))
            .padding(rem(1))
            .cursor(CssCursor::Pointer),
        IF!(active => {
            s()
                .background_color(Color::Background)
                .color(Color::SurfaceLight5_90)
        }),
        s()
            .hover()
            .background_color(Color::BackgroundLight1),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
            At::from("data-section") => title,
        },
        title,
    ]
}

#[view]
fn sections_container(root_base_url: &Url) -> Node<Msg> {
    div![
        C!["sections-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        general_section(root_base_url),
        player_section(),
        streaming_server_section(),
    ]
}

#[view]
fn general_section(root_base_url: &Url) -> Node<Msg> {
    let email = "Anonymous user";
    let options = vec![
        option_container(Some(s().height(rem(6))), vec![
            div![
                C!["avatar-container"],
                s()
                    .background_image(format!(r#"url("{}")"#, global::image_url("anonymous.png")).as_str())
                    .align_self(CssAlignSelf::Stretch)
                    .background_clip("content-box")
                    .background_origin("content-box")
                    .background_position(CssBackgroundPosition::Center)
                    .background_repeat(CssBackgroundRepeat::NoRepeat)
                    .background_size("cover")
                    .border_radius(pc(50))
                    .flex(CssFlex::None)
                    .margin_right(rem(1))
                    .opacity("0.9")
                    .width(rem(6)),
            ],
            div![
                C!["email-logout-container"],
                s()
                    .align_self(CssAlignSelf::Stretch)
                    .display(CssDisplay::Flex)
                    .flex("1")
                    .flex_direction(CssFlexDirection::Column)
                    .padding("1rem 0"),
                div![
                    C!["email-label-container"],
                    s()
                        .flex("1 0 auto")
                        .align_items(CssAlignItems::Center)
                        .display(CssDisplay::Flex)
                        .flex_direction(CssFlexDirection::Row),
                    attrs!{
                        At::Title => email,
                    },
                    div![
                        C!["email-label"],
                        s()
                            .color(Color::SurfaceLight5_90)
                            .flex("1")
                            .font_size(rem(1.1))
                            .max_height(em(2.4)),
                        email,
                    ]
                ]
            ],
            a![
                C!["user-panel-container", "button-container"],
                s()
                    .align_items(CssAlignItems::Center)
                    .background_color(Color::Accent3)
                    .display(CssDisplay::Flex)
                    .flex(CssFlex::None)
                    .flex_direction(CssFlexDirection::Row)
                    .height(rem(3.5))
                    .margin_left(rem(1))
                    .width(rem(10)),
                s()
                    .hover()
                    .background_color(Color::Accent3Light1),
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "User Panel",
                    At::Target => "_blank",
                    At::Href => "https://www.stremio.com/acc-settings",
                },
                div![
                    C!["user-panel-label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex("1")
                        .font_weight("500")
                        .max_height(rem(2.4))
                        .padding("0 0.5rem")
                        .text_align(CssTextAlign::Center),
                    "User Panel",
                ]
            ],
        ]),
        option_container(None, vec![
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
                    At::Title => "Log in / Sign up",
                    At::Href => RootUrls::new(root_base_url).intro(),
                },
                div![
                    C!["label"],
                    s()
                        .font_weight("500")
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Log in / Sign up",
                ],
            ]
        ]),
        option_container(None, vec![
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
                    "Interface language",
                ]                   
            ],
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
                    "English"
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
        ]),
        option_container(None, vec![
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
                    "Trakt Scrobbling",
                ]                   
            ],
            div![
                C!["option-input-container", "button-container", "disabled"],
                s()
                    .background_color(Color::Accent3)
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                s()
                    .hover()
                    .background_color(Color::Accent3Light1),
                attrs!{
                    At::TabIndex => -1,
                    At::Title => "Authenticate",
                    At::Href => RootUrls::new(root_base_url).intro(),
                },
                svg![
                    C!["icon"],
                    s()
                        .fill(Color::SurfaceLight5_90)
                        .flex(CssFlex::None)
                        .height(rem(1.5))
                        .margin_right(rem(0.5))
                        .width(rem(1.5))
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 1024 1024",
                        At::from("icon") => "ic_trakt",
                    },
                    path![
                        attrs!{
                            At::D => "M180.706 648.433l-30.118-36.744 487.605-487.906c-37.906-12.871-81.568-20.301-126.966-20.301-224.885 0-407.191 182.305-407.191 407.191 0 92.685 30.967 178.137 83.119 246.575l-0.727-0.994 213.835-216.847c36.442 37.045 70.174 72.282 104.809 107.219l203.595 203.595c8.433 8.433 15.962 17.468 26.504 4.518l-343.642-339.727-196.367 202.089-33.431-21.685c80.113-80.715 157.816-158.72 240.941-240.941 8.433 10.541 16.264 21.986 25.6 31.624l329.487 327.078c7.831 8.132 16.264 21.685 24.998 4.518l-387.313-386.711z",
                        },
                    ],
                    path![
                        attrs!{
                            At::D => "M701.44 147.878c-3.234-2.373-7.294-3.798-11.686-3.798-6.376 0-12.050 3.002-15.688 7.669l-0.033 0.044c-17.468 18.974-36.141 36.744-54.212 54.814l-189.44 187.633 388.819 388.819c4.216-5.12 9.336-10.541 13.854-16.264 45.49-57.694 76.366-128.892 85.933-206.685l0.203-2.030c1.528-13.351 2.4-28.824 2.4-44.501 0-157.684-88.195-294.77-217.948-364.618l-2.203-1.084zM457.487 400.866l237.026-234.616 23.191 21.986-237.929 237.327zM524.649 471.341l-25.6-23.191 206.607-204.8 20.179 27.708c-65.054 64.753-132.518 132.216-201.186 200.282z",
                        },
                    ],
                    path![
                        attrs!{
                            At::D => "M400.264 606.268l-186.127 185.525c98.184 120.471 306.598 171.369 465.92 92.762l-272.866-271.059z",
                        },
                    ],
                    path![
                        attrs!{
                            At::D => "M512 0c-282.77 0-512 229.23-512 512s229.23 512 512 512c282.77 0 512-229.23 512-512v0c0-282.77-229.23-512-512-512v0zM512 974.005c-255.158 0-462.005-206.847-462.005-462.005s206.847-462.005 462.005-462.005c255.158 0 462.005 206.847 462.005 462.005v-0c-0.343 255.021-206.985 461.662-461.972 462.005l-0.032 0z",
                        },
                    ],
                ],
                div![
                    C!["label"],
                    s()
                        .font_weight("500")
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Authenticate",
                ],
            ]
        ]),
        option_container(None, vec![
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
                    "Facebook",
                ]                   
            ],
            div![
                C!["option-input-container", "button-container", "disabled"],
                s()
                    .background_color(Color::Accent3)
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                s()
                    .hover()
                    .background_color(Color::Accent3Light1),
                attrs!{
                    At::TabIndex => -1,
                    At::Title => "Import",
                    At::Href => RootUrls::new(root_base_url).intro(),
                },
                svg![
                    C!["icon"],
                    s()
                        .fill(Color::SurfaceLight5_90)
                        .flex(CssFlex::None)
                        .height(rem(1.5))
                        .margin_right(rem(0.5))
                        .width(rem(1.5))
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 474 1024",
                        At::from("icon") => "ic_facebook",
                    },
                    path![
                        attrs!{
                            At::D => "M474.052 331.294h-161.431v-106.014c-0.245-1.731-0.385-3.731-0.385-5.764 0-23.952 19.417-43.369 43.369-43.369 0.665 0 1.326 0.015 1.984 0.045l-0.093-0.003h114.146v-176.188h-156.913c-174.381 0-213.835 131.012-213.835 214.739v116.555h-100.894v180.706h100.894v512h210.824v-512h143.059z",
                        },
                    ],
                ],
                div![
                    C!["label"],
                    s()
                        .font_weight("500")
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Import",
                ],
            ]
        ]),
        option_container(None, vec![
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
                    "Calendar",
                ]                   
            ],
            div![
                C!["option-input-container", "button-container", "disabled"],
                s()
                    .background_color(Color::Accent3)
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                s()
                    .hover()
                    .background_color(Color::Accent3Light1),
                attrs!{
                    At::TabIndex => -1,
                    At::Title => "Subscribe",
                    At::Href => RootUrls::new(root_base_url).intro(),
                },
                svg![
                    C!["icon"],
                    s()
                        .fill(Color::SurfaceLight5_90)
                        .flex(CssFlex::None)
                        .height(rem(1.5))
                        .margin_right(rem(0.5))
                        .width(rem(1.5))
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 1091 1024",
                        At::from("icon") => "ic_calendar",
                    },
                    path![
                        attrs!{
                            At::D => "M933.647 115.652h-65.355v-52.104c0-36.095-29.261-65.355-65.355-65.355s-65.355 29.261-65.355 65.355h0v52.104h-396.047v-52.104c0-36.095-29.261-65.355-65.355-65.355s-65.355 29.261-65.355 65.355v0 52.104h-53.007c-0.543-0.007-1.184-0.011-1.826-0.011-85.318 0-154.641 68.487-155.989 153.484l-0.002 0.127v602.353c2.016 84.597 71.073 152.406 155.968 152.406 0.65 0 1.299-0.004 1.947-0.012l-0.098 0.001h775.831c0.543 0.007 1.184 0.011 1.826 0.011 85.318 0 154.641-68.487 155.989-153.484l0.002-0.127v-602.353c-2.016-84.597-71.073-152.406-155.968-152.406-0.65 0-1.299 0.004-1.947 0.012l0.098-0.001zM993.882 870.4c0 33.267-26.968 60.235-60.235 60.235v0h-775.831c-33.267 0-60.235-26.968-60.235-60.235v0-458.089h896.301zM632.471 820.706h204.499c17.563-0.169 31.756-14.361 31.925-31.909l0-0.016v-204.499c0-17.632-14.293-31.925-31.925-31.925v0h-204.499c-0.090-0.001-0.196-0.001-0.303-0.001-17.465 0-31.624 14.158-31.624 31.624 0 0.106 0.001 0.213 0.002 0.319l-0-0.016v204.499c0 17.632 14.293 31.925 31.925 31.925v0z",
                        },
                    ],
                ],
                div![
                    C!["label"],
                    s()
                        .font_weight("500")
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Subscribe",
                ],
            ]
        ]),
        option_container(None, vec![
            div![
                C!["option-input-container", "link-container", "button-container", "disabled"],
                s()
                    .flex("0 1 auto")
                    .padding("1rem 0")
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                s()
                    .style_other(":hover > .label")
                    .text_decoration(CssTextDecoration::Underline),
                attrs!{
                    At::TabIndex => -1,
                    At::Title => "Export user data",
                },
                div![
                    C!["label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Export user data",
                ]   
            ]
        ]),
        option_container(None, vec![
            a![
                C!["option-input-container", "link-container", "button-container"],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Contact support",
                    At::Target => "_blank",
                    At::Href => "https://stremio.zendesk.com/hc/en-us",
                },
                div![
                    C!["label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Contact support",
                ]   
            ]
        ]),
        option_container(None, vec![
            a![
                C!["option-input-container", "link-container", "button-container"],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Source code",
                    At::Target => "_blank",
                    At::Href => "https://github.com/stremio/stremio-web/tree/deb73b6f6f02185bf680fa40cc8023af2060d5c6",
                },
                div![
                    C!["label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Source code",
                ]   
            ]
        ]),
        option_container(None, vec![
            a![
                C!["option-input-container", "link-container", "button-container"],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Terms of Service",
                    At::Target => "_blank",
                    At::Href => "https://www.stremio.com/tos",
                },
                div![
                    C!["label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Terms of Service",
                ]   
            ]
        ]),
        option_container(Some(s().margin_bottom("0")), vec![
            a![
                C!["option-input-container", "link-container", "button-container"],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Privacy Policy",
                    At::Target => "_blank",
                    At::Href => "https://www.stremio.com/privacy",
                },
                div![
                    C!["label"],
                    s()
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Privacy Policy",
                ]   
            ]
        ]),
    ];
    section_container("General", true, options)
}

#[view]
fn player_section() -> Node<Msg> {
    let options = vec![

    ];
    section_container("Player", true, options)
}

#[view]
fn streaming_server_section() -> Node<Msg> {
    let options = vec![

    ];
    section_container("Streaming Server", false, options)
}

#[view]
fn section_container(title: &str, bottom_border: bool, options: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        C!["section-container"],
        IF!(bottom_border => {
            s()
                .border_bottom("thin solid hsla(224.3,42.1%,66%,0.9)")
        }),
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .overflow(CssOverflow::Visible)
            .padding("3rem 0"),
        div![
            C!["section-title"],
            s()
                .align_self(CssAlignSelf::Stretch)
                .color(Color::SurfaceLight5_90)
                .flex(CssFlex::None)
                .font_size(rem(1.8))
                .line_height(rem(3.4))
                .margin_bottom(rem(1)),
            title,
        ],
        options,
    ]
}

#[view]
fn option_container(extra_style: Option<Style>, content: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        C!["option-container"],
        s()
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .margin_bottom(rem(2))
            .max_width(rem(35))
            .overflow(CssOverflow::Visible),
        extra_style,
        content,
    ]
}
