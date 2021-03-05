use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use std::rc::Rc;
use stremio_core::types::profile::User;
use stremio_core::runtime::msg::{Action, ActionCtx, Msg as CoreMsg};
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!"); })
}

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
    Logout
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Logout => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::Logout
            )))));
        }
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
    let user = context.core_model.ctx.profile.auth.as_ref().map(|auth| &auth.user);
    div![
        C!["settings-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(pc(100))
            .width(pc(100)),            
        side_menu_container(model.active_side_menu_button),
        sections_container(&context.root_base_url, user),
    ]
}

#[view]
fn side_menu_container(active_button: SideMenuButton) -> Node<Msg> {
    // @TODO: https://academind.com/tutorials/scroll-aware-navigation/ ?

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
fn sections_container(root_base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["sections-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        general_section(root_base_url, user),
        player_section(),
        streaming_server_section(),
    ]
}

#[view]
fn general_section(root_base_url: &Url, user: Option<&User>) -> Node<Msg> {
    let email = if let Some(user) = user {
        &user.email
    } else {
        "Anonymous user"
    };
    let login_button_title = if user.is_some() { "Log out "} else { "Log in / Sign up" };

    let avatar = user.and_then(|user| user.avatar.as_ref());
    let user_image_url: Cow<_> = match (user, avatar) {
        (None, _) => global::image_url("anonymous.png").into(),
        (Some(_), None) => global::image_url("default_avatar.png").into(),
        (Some(_), Some(avatar)) => avatar.into(),
    };
    
    let options = nodes![
        option_container(Some(s().height(rem(6))), vec![
            div![
                C!["avatar-container"],
                s()
                    .background_image(format!(r#"url("{}")"#, user_image_url).as_str())
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
                ],
                IF!(user.is_some() => {
                    a![
                        C!["logout-button-container", "button-container"],
                        s()
                            .flex("0 1 50%")
                            .align_items(CssAlignItems::Center)
                            .display(CssDisplay::Flex)
                            .flex_direction(CssFlexDirection::Row)
                            .cursor(CssCursor::Pointer),
                        s()
                            .style_other(":hover .logout-label")
                            .color(Color::SurfaceLight5_90)
                            .text_decoration(CssTextDecoration::Underline),
                        attrs!{
                            At::TabIndex => 0,
                            At::Title => "Log out",
                            At::Href => RootUrls::new(root_base_url).intro(),
                        },
                        user.map(|_| ev(Ev::Click, |_| Msg::Logout)),
                        div![
                            C!["logout-label"],
                            s()
                                .color(Color::Surface90)
                                .flex("1")
                                .max_height(em(1.2)),
                            "Log out",
                        ],
                    ]
                }),
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
        IF!(user.is_none() => { option_container(None, vec![
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
        ])}),
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
                    "Subtitles language",
                ]                   
            ],
            div![
                C!["option-input-container", "multiselect-container", "label-container", "button-container"],
                s()
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .background_color(Color::Background)
                    .overflow(CssOverflow::Visible)
                    .position(CssPosition::Relative)
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => -1,
                },
                on_click_not_implemented(),
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
                    "Subtitles size",
                ]                   
            ],
            div![
                C!["option-input-container", "multiselect-container", "label-container", "button-container"],
                s()
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .background_color(Color::Background)
                    .overflow(CssOverflow::Visible)
                    .position(CssPosition::Relative)
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => -1,
                },
                on_click_not_implemented(),
                div![
                    C!["label"],
                    s()
                        .line_height(rem(1.5))
                        .max_height(rem(1.5))
                        .color(Color::SecondaryVariant1_90)
                        .flex("1")
                        .font_weight("500"),
                    "100%"
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
                    "Subtitles text color",
                ]                   
            ],
            div![
                C!["option-input-container", "color-input-container", "button-container"],
                s()
                    .background_color("rgb(255, 255, 255)")
                    .padding("1.75rem 1rem")
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .position(CssPosition::Relative)
                    .z_index("0")
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "#FFFFFFFF",
                },
                on_click_not_implemented(),
            ],
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
                    "Subtitles background color",
                ]                   
            ],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Transparent",
                },
                on_click_not_implemented(),
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
            ],
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
                    "Subtitles outline color",
                ]                   
            ],
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
                attrs!{
                    At::TabIndex => 0,
                    At::Title => "Transparent",
                },
                on_click_not_implemented(),
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
            ],
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
                    "Auto-play next episode",
                ]                   
            ],
            div![
                C!["option-input-container", "checkbox-container", "button-container"],
                s()
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => 0,
                },
                on_click_not_implemented(),
                svg![
                    C!["icon"],
                    s()
                        .height(rem(1.5))
                        .width(rem(1.5))
                        .display(CssDisplay::Block)
                        .fill(Color::SurfaceLight5)
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 1024 1024",
                        At::from("icon") => "ic_box_empty",
                    },
                    path![
                        attrs!{
                            At::D => "M843.294 180.706v662.588h-662.588v-662.588h662.588zM1024 0h-1024v1024h1024v-1024z",   
                        }
                    ],
                ]
            ],
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
                    "Play in background",
                ]                   
            ],
            div![
                C!["option-input-container", "checkbox-container", "button-container", "disabled"],
                s()
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => -1,
                },
                on_click_not_implemented(),
                svg![
                    C!["icon"],
                    s()
                        .height(rem(1.5))
                        .width(rem(1.5))
                        .display(CssDisplay::Block)
                        .background_color(Color::PrimaryVariant1)
                        .fill(Color::SurfaceLight5)
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 100 100",
                    },
                    svg![
                        attrs!{
                            At::ViewBox => "0 0 1331 1024",
                            At::X => 10,
                            At::Y => 10,
                            At::Width => 80,
                            At::Height => 80,
                            At::from("icon") => "ic_check",
                        },
                        path![
                            attrs!{
                                At::D => "M545.129 1024c-40.334-0.026-76.847-16.363-103.306-42.769l-398.755-397.551c-24.752-26.158-39.97-61.56-39.97-100.516 0-80.839 65.533-146.372 146.372-146.372 38.806 0 74.085 15.101 100.281 39.748l-0.075-0.070 288.226 286.118 536.395-612.593c27.002-30.81 66.432-50.158 110.381-50.158 80.929 0 146.535 65.606 146.535 146.535 0 36.98-13.698 70.761-36.298 96.544l0.144-0.168-639.699 731.256c-25.909 29.451-63.15 48.401-104.838 49.987l-0.272 0.008z",   
                            }
                        ],
                    ],
                ]
            ],
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
                    "Play in external player",
                ]                   
            ],
            div![
                C!["option-input-container", "checkbox-container", "button-container", "disabled"],
                s()
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => -1,
                },
                on_click_not_implemented(),
                svg![
                    C!["icon"],
                    s()
                        .height(rem(1.5))
                        .width(rem(1.5))
                        .display(CssDisplay::Block)
                        .fill(Color::SurfaceLight5)
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 1024 1024",
                        At::from("icon") => "ic_box_empty",
                    },
                    path![
                        attrs!{
                            At::D => "M843.294 180.706v662.588h-662.588v-662.588h662.588zM1024 0h-1024v1024h1024v-1024z",   
                        }
                    ],
                ]
            ],
        ]),
        option_container(Some(s().margin_bottom("0")), vec![
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
                    "Hardware-accelerated decoding",
                ]                   
            ],
            div![
                C!["option-input-container", "checkbox-container", "button-container", "disabled"],
                s()
                    .justify_content(CssJustifyContent::Center)
                    .padding(rem(1))
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row)
                    .pointer_events("none")
                    .cursor(CssCursor::Pointer),
                attrs!{
                    At::TabIndex => -1,
                },
                on_click_not_implemented(),
                svg![
                    C!["icon"],
                    s()
                        .height(rem(1.5))
                        .width(rem(1.5))
                        .display(CssDisplay::Block)
                        .fill(Color::SurfaceLight5)
                        .overflow(CssOverflow::Visible),
                    attrs!{
                        At::ViewBox => "0 0 1024 1024",
                        At::from("icon") => "ic_box_empty",
                    },
                    path![
                        attrs!{
                            At::D => "M843.294 180.706v662.588h-662.588v-662.588h662.588zM1024 0h-1024v1024h1024v-1024z",   
                        }
                    ],
                ]
            ],
        ]),
    ];
    section_container("Player", true, options)
}

#[view]
fn streaming_server_section() -> Node<Msg> {
    let options = vec![
        option_container(None, vec![
            div![
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
                    At::Title => "Reload",
                },
                on_click_not_implemented(),
                div![
                    C!["label"],
                    s()
                        .font_weight("500")
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_grow("0")
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    "Reload",
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
                    "Status",
                ]                   
            ],
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
            ],
        ]),
        option_container(Some(s().margin_bottom("0")), vec![
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
                    "Url",
                ]                   
            ],
            div![
                C!["option-input-container", "configure-input-container"],
                s()
                    .padding("0")
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex("1 1 50%")
                    .flex_direction(CssFlexDirection::Row),
                div![
                    C!["label"],
                    s()
                        .flex_grow("1")
                        .padding("0 1rem")
                        .text_overflow("ellipsis")
                        .white_space(CssWhiteSpace::Pre)
                        .color(Color::SurfaceLight5_90)
                        .flex_basis(CssFlexBasis::Auto)
                        .flex_shrink("1")
                        .line_height(rem(1.5)),
                    attrs!{
                        At::Title => "http://127.0.0.1:11470/",
                    },
                    "http://127.0.0.1:11470/",
                ],
                div![
                    C!["configure-button-container", "button-container"],
                    s()
                        .align_items(CssAlignItems::Center)
                        .background_color(Color::Accent3)
                        .display(CssDisplay::Flex)
                        .flex(CssFlex::None)
                        .flex_direction(CssFlexDirection::Row)
                        .height(rem(3))
                        .justify_content(CssJustifyContent::Center)
                        .width(rem(3))
                        .cursor(CssCursor::Pointer),
                    s()
                        .hover()
                        .background_color(Color::Accent3Light1),
                    attrs!{
                        At::TabIndex => 0,
                        At::Title => "Configure server url",
                    },
                    on_click_not_implemented(),
                    svg![
                        C!["icon"],
                        s()
                            .fill(Color::SurfaceLight5_90)
                            .flex(CssFlex::None)
                            .height(rem(1))
                            .margin("0")
                            .width(rem(1))
                            .overflow(CssOverflow::Visible),
                        attrs!{
                            At::ViewBox => "0 0 1043 1024",
                            At::from("icon") => "ic_settings",
                        },
                        path![
                            attrs!{
                                At::D => "M791.492 901.421c-0.137 1.886-0.214 4.085-0.214 6.303 0 14.689 3.414 28.58 9.492 40.924l-0.242-0.544c1.442 2.027 2.306 4.553 2.306 7.281 0 5.548-3.572 10.262-8.542 11.967l-0.089 0.027c-37.735 21.585-81.411 40.158-127.33 53.451l-4.284 1.062c-2.114 1.002-4.593 1.587-7.209 1.587-7.903 0-14.559-5.341-16.556-12.61l-0.028-0.12c-20.88-43.535-64.606-73.060-115.229-73.060-26.819 0-51.703 8.287-72.23 22.44l0.428-0.279c-19.628 13.227-34.808 31.704-43.688 53.426l-0.284 0.786c-3.614 8.734-7.529 11.746-17.769 9.035-51.834-13.272-97.233-31.525-139.449-54.835l3.016 1.527c-14.758-7.831-8.734-16.866-5.12-26.805 4.846-12.398 7.654-26.752 7.654-41.762 0-32.050-12.804-61.11-33.576-82.344l0.021 0.021c-22.874-25.484-55.92-41.441-92.693-41.441-10.83 0-21.336 1.384-31.352 3.985l0.864-0.191h-5.722c-30.118 9.336-30.118 9.035-44.273-18.372-17.236-31.193-32.683-67.512-44.377-105.477l-1.101-4.152c-3.915-12.348-1.807-18.673 11.445-24.094 45.171-18.059 76.501-61.451 76.501-112.16 0-0.275-0.001-0.549-0.003-0.823l0 0.042c-0.157-51.84-32.003-96.203-77.176-114.748l-0.829-0.301c-13.553-4.819-15.962-10.842-12.047-23.793 13.962-48.504 31.914-90.674 54.24-130.036l-1.534 2.94c6.024-10.541 11.746-12.649 23.793-7.831 14.648 6.459 31.727 10.219 49.685 10.219 35.285 0 67.18-14.517 90.038-37.904l0.023-0.024c21.532-21.755 34.835-51.691 34.835-84.733 0-19.022-4.409-37.015-12.26-53.011l0.314 0.709c-4.216-9.638-3.012-15.059 6.024-20.48 39.702-23.013 85.609-42.536 133.977-56.195l4.263-1.029c13.252-3.614 14.758 5.12 18.372 13.252 16.261 41.325 53.282 71.221 97.87 77.036l0.614 0.065c6.241 1.121 13.425 1.762 20.759 1.762 40.852 0 77.059-19.886 99.469-50.507l0.242-0.347c7.452-9.232 13.404-20.047 17.264-31.809l0.204-0.718c3.012-8.433 8.132-9.939 16.264-8.132 52.584 13.65 98.681 32.83 141.232 57.456l-2.691-1.437c9.336 5.12 8.433 11.144 4.819 19.576-6.604 14.774-10.451 32.016-10.451 50.158 0 69.362 56.229 125.591 125.591 125.591 18.623 0 36.299-4.053 52.195-11.326l-0.784 0.321c10.24-4.518 15.962-3.012 21.384 6.927 22.212 37.657 40.917 81.17 53.87 127.095l0.944 3.916c2.711 10.24 0 15.36-10.24 19.878-46.208 16.823-78.61 60.371-78.61 111.487 0 0.299 0.001 0.599 0.003 0.898l-0-0.046c-0.106 1.871-0.166 4.060-0.166 6.264 0 49.766 30.792 92.34 74.362 109.71l0.797 0.28c12.951 6.024 16.264 11.746 12.047 25.6-14.446 47.781-32.562 89.199-54.858 127.907l1.55-2.918c-5.421 10.24-10.842 12.348-22.287 8.132-14.209-5.966-30.724-9.432-48.048-9.432-45.354 0-85.159 23.756-107.651 59.503l-0.31 0.527c-11.029 16.816-17.591 37.422-17.591 59.561 0 1.826 0.045 3.642 0.133 5.446l-0.010-0.254zM520.433 711.68c109.44-1.529 197.571-90.604 197.571-200.264 0-110.613-89.669-200.282-200.282-200.282s-200.282 89.669-200.282 200.282c0 0.205 0 0.411 0.001 0.616l-0-0.032c0.498 110.402 90.11 199.707 200.582 199.707 1.166 0 2.329-0.010 3.49-0.030l-0.175 0.002z",
                            }
                        ]
                    ]
                ]
            ]
        ]),
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
