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
    control::{label, dropdown, connect_button, link_label, large_button, url, status}
};
use web_sys::HtmlElement;

#[view]
pub fn streaming_server_section(section_ref: &ElRef<HtmlElement>) -> Node<Msg> {
    let options = vec![
        section_option(None, vec![
            large_button("Reload", None)
        ]),
        section_option(None, vec![
            label("Status"),
            status()
        ]),
        section_option(Some(s().margin_bottom("0")), vec![
            label("Url"),
            url("http://127.0.0.1:11470/", "Configure server url")
        ]),
    ];
    section("Streaming Server", false, section_ref, options)
}
