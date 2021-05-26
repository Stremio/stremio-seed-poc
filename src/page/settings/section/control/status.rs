use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::models::common::Loadable;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use crate::page::settings::Msg;
use stremio_core::models::streaming_server::StreamingServer;

#[view]
pub fn status(streaming_server: &StreamingServer) -> Node<Msg> {
    let status = match &streaming_server.settings {
        Loadable::Loading => "Loading...",
        Loadable::Ready(_) => "Online",
        Loadable::Err(error) => {
            error!(error.code(), error.message());
            "Error"
        }
    };

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
            status,
        ]
    ]
}
