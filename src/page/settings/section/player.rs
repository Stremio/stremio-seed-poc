use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use stremio_core::types::profile::{User, Settings};
use crate::Urls as RootUrls;
use crate::styles::{self, themes::Color, global};
use crate::page::settings::{Msg, UpdateSettingsMsg};
use crate::page::settings::section::{
    section_option,
    section,
    control::{label, dropdown, connect_button, link_label, color_picker, checkbox}
};
use web_sys::Element;

#[view]
pub fn player_section(settings: &Settings, section_ref: &ElRef<Element>) -> Node<Msg> {
    let options = vec![
        section_option(None, vec![
            label("Subtitles language"),
            // @TODO `settings.interface_language` returns `eng` instead of `English`
            dropdown(&settings.subtitles_language)
        ]),
        section_option(None, vec![
            label("Subtitles size"),
            // @TODO `settings.subtitles_size` returns `100` instead of `100%`
            dropdown(&settings.subtitles_size.to_string())
        ]),
        section_option(None, vec![
            label("Subtitles text color"),
            {
                let color = settings.subtitles_text_color.as_str();
                let arg = if color == "#00000000" { None } else { Some((color, color)) };
                color_picker(arg)
            }
        ]),
        section_option(None, vec![
            label("Subtitles background color"),
            {
                let color = settings.subtitles_background_color.as_str();
                let arg = if color == "#00000000" { None } else { Some((color, color)) };
                color_picker(arg)
            }
        ]),
        section_option(None, vec![
            label("Subtitles outline color"),
            {
                let color = settings.subtitles_outline_color.as_str();
                let arg = if color == "#00000000" { None } else { Some((color, color)) };
                color_picker(arg)
            }
        ]),
        section_option(None, vec![
            label("Auto-play next episode"),
            checkbox(
                settings.binge_watching, 
                { 
                    let new_binge_watching = not(settings.binge_watching);  
                    ev(Ev::Click, move |_| Msg::UpdateSettings(UpdateSettingsMsg::BingeWatching(new_binge_watching)))
                }, 
                true
            ),
        ]),
        section_option(None, vec![
            label("Play in background"),
            checkbox(settings.play_in_background, None, false),
        ]),
        section_option(None, vec![
            label("Play in external player"),
            checkbox(settings.play_in_external_player, None, false),
        ]),
        section_option(Some(s().margin_bottom("0")), vec![
            label("Hardware-accelerated decoding"),
            checkbox(settings.hardware_decoding, None, false),
        ]),
    ];
    section("Player", true, section_ref, options)
}
