use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::types::profile::User;
use crate::Urls as RootUrls;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;

#[view]
pub fn user_info(root_base_url: &Url, user: Option<&User>) -> Vec<Node<Msg>> {
    let email = if let Some(user) = user {
        &user.email
    } else {
        "Anonymous user"
    };
    let avatar = user.and_then(|user| user.avatar.as_ref());
    let user_image_url: Cow<_> = match (user, avatar) {
        (None, _) => global::image_url("anonymous.png").into(),
        (Some(_), None) => global::image_url("default_avatar.png").into(),
        (Some(_), Some(avatar)) => avatar.into(),
    };

    vec![
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
    ]
}
