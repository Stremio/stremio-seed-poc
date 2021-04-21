use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::borrow::Cow;
use std::rc::Rc;
use std::collections::HashMap;
use url::Url as CoreUrl;
use stremio_core::types::profile::User;
use stremio_core::runtime::msg::{Action, ActionStreamingServer, ActionCtx, Msg as CoreMsg};
use stremio_core::models::common::Loadable;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls, Events};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use web_sys::{
    ScrollIntoViewOptions, 
    ScrollBehavior, 
    IntersectionObserver, 
    IntersectionObserverInit, 
    IntersectionObserverEntry
};
use js_sys::Array;

mod side_menu;
use side_menu::side_menu;

mod section;
use section::{sections, SectionRefs, Section};

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

    orders
        .after_next_render(|_| Msg::Rendered);

    let model = model.get_or_insert_with(move || {
        Model {
            active_section: Section::General,
            section_ratios: vec![
                (Section::General, 0.),
                (Section::Player, 0.),
                (Section::StreamingServer, 0.),
            ].into_iter().collect(),
            section_refs: SectionRefs::default(),
            observer: None,
            observer_callback: None,
            page_change_sub_handle: None,
        }
    });
    model.page_change_sub_handle = Some(orders.subscribe_with_handle(|events| {
        if let Events::PageChanged(page_id) = events {
            return Some(Msg::PageChanged(page_id))
        }
        None
    }));
    Some(PageId::Settings)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    active_section: Section,
    section_ratios: HashMap<Section, f64>,
    section_refs: SectionRefs,
    observer: Option<IntersectionObserver>,
    observer_callback: Option<Closure<dyn Fn(Vec<JsValue>)>>,
    page_change_sub_handle: Option<SubHandle>,
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
    Rendered,
    Observed(Vec<IntersectionObserverEntry>),
    PageChanged(PageId),
    Logout,
    MenuButtonClicked(Section),
    UpdateSettings(UpdateSettingsMsg),
    ReloadStreamingServer,
}

pub enum UpdateSettingsMsg {
    InterfaceLanguage(String),
    StreamingServerUrl(CoreUrl),
    BingeWatching(bool),
    PlayInBackground(bool),
    PlayInExternalPlayer(bool),
    HardwareDecoding(bool),
    SubtitlesLanguage(String),
    SubtitlesSize(u8),
    SubtitlesFont(String),
    SubtitlesBold(bool),
    SubtitlesOffset(u8),
    SubtitlesTextColor(String),
    SubtitlesBackgroundColor(String),
    SubtitlesOutlineColor(String),
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            orders.skip();

            let sender = orders.msg_sender();
            let callback = move |entries: Vec<JsValue>| {
                let entries = entries
                    .into_iter()
                    .map(IntersectionObserverEntry::from)
                    .collect();
                sender(Some(Msg::Observed(entries)));
            };
            let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(Vec<JsValue>)>);

            let mut options = IntersectionObserverInit::new();
            options.root(Some(&model.section_refs.container.get().unwrap()));
            let thresholds = (0..=10_u8)
                .map(|value| JsValue::from(f64::from(value) / 10.))
                .collect::<Array>();
            options.threshold(&JsValue::from(thresholds));

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(), 
                &options
            ).unwrap();

            observer.observe(&model.section_refs.general.get().unwrap());
            observer.observe(&model.section_refs.player.get().unwrap());
            observer.observe(&model.section_refs.streaming_server.get().unwrap());

            model.observer = Some(observer);
            model.observer_callback = Some(callback);
        }
        Msg::Observed(entries) => {
            let old_active_section = model.active_section;
            for entry in entries {
                let target_el = entry.target();
                let ratio = entry.intersection_ratio();

                let section = match &target_el {
                    target_el if Some(target_el) == model.section_refs.general.get().as_ref() => {
                        Section::General
                    }
                    target_el if Some(target_el) == model.section_refs.player.get().as_ref() => {
                        Section::Player
                    }
                    target_el if Some(target_el) == model.section_refs.streaming_server.get().as_ref() => {
                        Section::StreamingServer
                    }
                    _ => {
                        orders.skip();
                        return
                    }
                };
                *model.section_ratios.get_mut(&section).unwrap() = ratio;
            }
            model.active_section = model
                .section_ratios
                .iter()
                .max_by(|(_, ratio_a), (_, ratio_b)| ratio_a.partial_cmp(ratio_b).unwrap())
                .map(|(section, _)| *section)
                .unwrap();
            if old_active_section == model.active_section {
                orders.skip();
            }
        }
        Msg::PageChanged(page_id) => {
            if page_id != PageId::Settings {
                if let Some(observer) = &model.observer {
                    observer.disconnect();
                }
                model.observer = None;
                model.observer_callback = None;
                model.page_change_sub_handle = None;
            }
        }
        Msg::Logout => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::Logout
            )))));
        }
        Msg::MenuButtonClicked(section) => {
            let section_ref = match section {
                Section::General => &model.section_refs.general,
                Section::Player => &model.section_refs.player,
                Section::StreamingServer => &model.section_refs.streaming_server,
            };
            let mut options = ScrollIntoViewOptions::new();
            // @TODO: Does it work on Safari?
            options.behavior(ScrollBehavior::Smooth);
            section_ref.get().unwrap().scroll_into_view_with_scroll_into_view_options(&options);
        }
        Msg::UpdateSettings(msg) => {
            use UpdateSettingsMsg::*;
            let mut settings = context.core_model.ctx.profile.settings.to_owned();
            match msg {
                InterfaceLanguage(value) => settings.interface_language = value,
                StreamingServerUrl(value) => settings.streaming_server_url = value,
                BingeWatching(value) => settings.binge_watching = value,
                PlayInBackground(value) => settings.play_in_background = value,
                PlayInExternalPlayer(value) => settings.play_in_external_player = value,
                HardwareDecoding(value) => settings.hardware_decoding = value,
                SubtitlesLanguage(value) => settings.subtitles_language = value,
                SubtitlesSize(value) => settings.subtitles_size = value,
                SubtitlesFont(value) => settings.subtitles_font = value,
                SubtitlesBold(value) => settings.subtitles_bold = value,
                SubtitlesOffset(value) => settings.subtitles_offset = value,
                SubtitlesTextColor(value) => settings.subtitles_text_color = value,
                SubtitlesBackgroundColor(value) => settings.subtitles_background_color = value,
                SubtitlesOutlineColor(value) => settings.subtitles_outline_color = value,
            }
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::UpdateSettings(settings)
            )))));
        }
        Msg::ReloadStreamingServer => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::StreamingServer(
                ActionStreamingServer::Reload,
            )))));
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Vec<Node<RootMsg>> {
    basic_layout(BasicLayoutArgs {
        page_content: settings_content(model, context).map_msg(msg_mapper),
        container_class: "settings-container",
        context,
        page_id,
        search_args: None,
        modal: None,
    })
}

#[view]
fn settings_content<'a>(model: &Model, context: &Context) -> Node<Msg> {
    let user = context.core_model.ctx.profile.auth.as_ref().map(|auth| &auth.user);
    let settings = &context.core_model.ctx.profile.settings;
    
    let streaming_server_settings = &context.core_model.streaming_server.settings;
    let server_version = match streaming_server_settings {
        Loadable::Ready(settings) => Some(settings.server_version.as_str()),
        _ => None
    };

    div![
        C!["settings-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(pc(100))
            .width(pc(100)),            
        side_menu(model.active_section, server_version),
        sections(settings, &context.root_base_url, user, &model.section_refs, &context.core_model.streaming_server),
    ]
}


