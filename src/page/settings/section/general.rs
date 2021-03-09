use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::types::profile::User;
use crate::Urls as RootUrls;
use crate::styles::{self, themes::Color, global};
use crate::page::settings::Msg;
use crate::page::settings::section::{
    section_option,
    section,
    control::{label, dropdown, connect_button, link_label, button_label, user_info, large_button}
};
use web_sys::HtmlElement;

#[view]
pub fn general_section(root_base_url: &Url, user: Option<&User>, section_ref: &ElRef<HtmlElement>) -> Node<Msg> {
    let options = nodes![
        section_option(Some(s().height(rem(6))), user_info(root_base_url, user)),
        IF!(user.is_none() => { section_option(None, vec![
            large_button("Log in / Sign up", RootUrls::new(root_base_url).intro())
        ])}),
        section_option(None, vec![
            label("Interface language"),
            dropdown("English")
        ]),
        section_option(None, vec![
            label("Trakt Scrobbling"),
            connect_button("Authenticate", "ic_trakt", "0 0 1024 1024", None, false, vec![
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
            ])
        ]),
        section_option(None, vec![
            label("Facebook"),
            connect_button("Import", "ic_facebook", "0 0 474 1024", None, false, vec![
                path![
                    attrs!{
                        At::D => "M474.052 331.294h-161.431v-106.014c-0.245-1.731-0.385-3.731-0.385-5.764 0-23.952 19.417-43.369 43.369-43.369 0.665 0 1.326 0.015 1.984 0.045l-0.093-0.003h114.146v-176.188h-156.913c-174.381 0-213.835 131.012-213.835 214.739v116.555h-100.894v180.706h100.894v512h210.824v-512h143.059z",
                    },
                ],
            ])
        ]),
        section_option(None, vec![
            label("Calendar"),
            connect_button("Subscribe", "ic_calendar", "0 0 1091 1024", None, false, vec![
                path![
                    attrs!{
                        At::D => "M933.647 115.652h-65.355v-52.104c0-36.095-29.261-65.355-65.355-65.355s-65.355 29.261-65.355 65.355h0v52.104h-396.047v-52.104c0-36.095-29.261-65.355-65.355-65.355s-65.355 29.261-65.355 65.355v0 52.104h-53.007c-0.543-0.007-1.184-0.011-1.826-0.011-85.318 0-154.641 68.487-155.989 153.484l-0.002 0.127v602.353c2.016 84.597 71.073 152.406 155.968 152.406 0.65 0 1.299-0.004 1.947-0.012l-0.098 0.001h775.831c0.543 0.007 1.184 0.011 1.826 0.011 85.318 0 154.641-68.487 155.989-153.484l0.002-0.127v-602.353c-2.016-84.597-71.073-152.406-155.968-152.406-0.65 0-1.299 0.004-1.947 0.012l0.098-0.001zM993.882 870.4c0 33.267-26.968 60.235-60.235 60.235v0h-775.831c-33.267 0-60.235-26.968-60.235-60.235v0-458.089h896.301zM632.471 820.706h204.499c17.563-0.169 31.756-14.361 31.925-31.909l0-0.016v-204.499c0-17.632-14.293-31.925-31.925-31.925v0h-204.499c-0.090-0.001-0.196-0.001-0.303-0.001-17.465 0-31.624 14.158-31.624 31.624 0 0.106 0.001 0.213 0.002 0.319l-0-0.016v204.499c0 17.632 14.293 31.925 31.925 31.925v0z",
                    },
                ],
            ])
        ]),
        section_option(None, vec![
            button_label("Export user data", None, false)
        ]),
        section_option(None, vec![
            link_label("Contact support", "https://stremio.zendesk.com/hc/en-us")
        ]),
        section_option(None, vec![
            link_label("Source code", "https://github.com/stremio/stremio-web/tree/deb73b6f6f02185bf680fa40cc8023af2060d5c6")
        ]),
        section_option(None, vec![
            link_label("Terms of Service", "https://www.stremio.com/tos")
        ]),
        section_option(Some(s().margin_bottom("0")), vec![
            link_label("Privacy Policy", "https://www.stremio.com/privacy")
        ]),
    ];
    section("General", true, section_ref, options)
}
