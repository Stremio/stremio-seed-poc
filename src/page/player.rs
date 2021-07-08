use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
use std::convert::{TryFrom, TryInto};
use enclose::enc;
use serde::Serialize;
use crate::{PageId, Context, Actions, Events};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::types::resource::{Stream, StreamSource};
use stremio_core::models::player::Selected as PlayerSelected;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg, Internal};
use js_sys::Reflect;

mod nav_bar;
mod control_bar;

use nav_bar::nav_bar;
use control_bar::control_bar;

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

    let stream: Stream = serde_json::from_str(url.next_hash_path_part()?).ok()?;

    load_player(stream.clone(), orders);

    orders.after_next_render(|_| Msg::Rendered);
    
    let mut model = model.get_or_insert_with(move || Model {
        base_url,
        video_ref: ElRef::new(),
        youtube: None,
        stream: None,
        page_change_sub_handle: orders.subscribe_with_handle(|events| {
            matches!(events, Events::PageChanged(page_id) if page_id != PageId::Player)
                .then(|| Msg::DestroyPlayer)
        }),
        playing: false,
    });
    model.stream = Some(stream);
    model.playing = false;
    Some(PageId::Player)
}

fn load_player(stream: Stream, orders: &mut impl Orders<Msg>) {
    let player_selected = PlayerSelected {
        stream,
        meta_request: None,
        stream_request: None,
        subtitles_path: None
    };
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::Player(player_selected),
    )))));
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    video_ref: ElRef<HtmlElement>,
    youtube: Option<Youtube>,
    stream: Option<Stream>,
    page_change_sub_handle: SubHandle,
    playing: bool,
}

pub struct Youtube {
    video_container: Rc<web_sys::HtmlElement>,
    api_script: web_sys::HtmlScriptElement,
    on_api_loaded: Closure<dyn Fn()>,
    on_api_error: Closure<dyn Fn()>,
    on_ready: Rc<Closure<dyn Fn()>>,
    player: Option<Player>,
    on_player_ready: Option<Closure<dyn Fn()>>,
    on_player_state_change: Option<Closure<dyn Fn(JsValue)>>,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn stream(self, stream: &Stream) -> Url {
        self.base_url().add_hash_path_part(serde_json::to_string(stream).unwrap())
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Rendered,
    YoutubeReady(Rc<HtmlElement>, String),
    YoutubePlayerStateChanged(YoutubePlayerState),
    DestroyPlayer,
    ToggleFullscreen,
    TogglePlay,
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            match &model.stream.as_ref().unwrap().source {
                StreamSource::YouTube { yt_id } => {
                    model.youtube = Some(init_youtube(&model.video_ref, yt_id.clone(), orders));
                }
                stream_source => error!("Unhandled stream source:", stream_source),
            }
        }
        Msg::YoutubeReady(video_container, yt_id) => {
            // -- player_vars --
            let player_vars = PlayerVars {
                autoplay: 1,
                cc_load_policy: 3,
                controls: 0,
                disablekb: 1,
                enablejsapi: 1,
                fs: 0,
                iv_load_policy: 3,
                r#loop: 0,
                modestbranding: 1,
                playsinline: 1,
                rel: 0
            };
            let player_vars = serde_wasm_bindgen::to_value(&player_vars).unwrap();

            // -- on_ready --
            let on_ready = || log!("Youtube player ready");
            let on_ready = Closure::wrap(Box::new(on_ready) as Box<dyn Fn()>);

            // -- on_state_change --
            let sender = orders.msg_sender();
            let on_state_change = move |event: JsValue| {
                let state = Reflect::get(&event, &"data".into()).unwrap().as_f64().unwrap() as i8;
                let state = YoutubePlayerState::try_from(state).unwrap();
                sender(Some(Msg::YoutubePlayerStateChanged(state)));
            };
            let on_state_change = Closure::wrap(Box::new(on_state_change) as Box<dyn Fn(JsValue)>);

            // -- events --
            let events = js_sys::Object::new();
            Reflect::set(&events, &"onReady".into(), on_ready.as_ref()).unwrap();
            Reflect::set(&events, &"onStateChange".into(), on_state_change.as_ref()).unwrap();

            // -- config --
            let config = js_sys::Object::new();
            Reflect::set(&config, &"width".into(), &"100%".into()).unwrap();
            Reflect::set(&config, &"height".into(), &"100%".into()).unwrap();
            Reflect::set(&config, &"videoId".into(), &yt_id.into()).unwrap();
            Reflect::set(&config, &"playerVars".into(), &player_vars).unwrap();
            Reflect::set(&config, &"events".into(), &events).unwrap();

            log!("Youtube config:", config);
            if let Some(youtube) = model.youtube.as_mut() {
                youtube.player = Some(Player::new(&video_container, config));
                youtube.on_player_ready = Some(on_ready);
                youtube.on_player_state_change = Some(on_state_change);
            }
        }
        Msg::DestroyPlayer => {
            if let Some(mut youtube) = model.youtube.take() {
                if let Some(player) = youtube.player.take() {
                    player.destroy();
                }
                youtube.video_container.remove();
                youtube.api_script.remove();
            }
        }
        Msg::YoutubePlayerStateChanged(state) => {
            match state {
                YoutubePlayerState::Playing => model.playing = true,
                YoutubePlayerState::Paused | YoutubePlayerState::Ended => model.playing = false,
                _ => (),
            }
            log!(state);
        }
        Msg::ToggleFullscreen => {
            orders.notify(Actions::ToggleFullscreen);
        }
        Msg::TogglePlay => {
            log!("toggle play");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum YoutubePlayerState {
    Unstarted,
    Ended,
    Playing,
    Paused,
    Buffering,
    Cued,
}

impl TryFrom<i8> for YoutubePlayerState {
    type Error = &'static str;

    fn try_from(state: i8) -> Result<Self, Self::Error> {
        match state {
            -1 => Ok(Self::Unstarted),
            0 => Ok(Self::Ended),
            1 => Ok(Self::Playing),
            2 => Ok(Self::Paused),
            3 => Ok(Self::Buffering),
            5 => Ok(Self::Cued),
            _ => Err("Unknown YT.PlayerState value")
        }
    }
}

fn init_youtube(video_ref: &ElRef<HtmlElement>, yt_id: String, orders: &mut impl Orders<Msg>) -> Youtube {
    let container = video_ref.get().expect("video container");

    // -- video_container --
    let video_container = document().create_element("div").unwrap().unchecked_into::<web_sys::HtmlElement>();
    let video_container_style = video_container.style();
    video_container_style.set_property("width", "100%").unwrap();
    video_container_style.set_property("height", "100%").unwrap();
    video_container_style.set_property("backgroundColor", "black").unwrap();
    let video_container = Rc::new(video_container);

    // -- api_script --
    let api_script = document().create_element("script").unwrap().unchecked_into::<web_sys::HtmlScriptElement>();
    api_script.set_type("text/javascript");
    api_script.set_src("https://www.youtube.com/iframe_api");

    // -- on_ready --
    let sender = orders.msg_sender();
    let on_ready = enc!((video_container) move || {
        sender(Some(Msg::YoutubeReady(video_container.clone(), yt_id.clone())));
    });
    let on_ready = Rc::new(Closure::wrap(Box::new(on_ready) as Box<dyn Fn()>));

    // -- on_api_loaded --
    let on_api_loaded = enc!((on_ready) move || {
        YT::ready(on_ready.as_ref().as_ref().unchecked_ref());
    });
    let on_api_loaded = Closure::wrap(Box::new(on_api_loaded) as Box<dyn Fn()>);
    api_script.set_onload(Some(on_api_loaded.as_ref().unchecked_ref()));

    // -- on_api_error --
    let on_api_error = || {
        error!("Youtube error");
    };
    let on_api_error = Closure::wrap(Box::new(on_api_error) as Box<dyn Fn()>);
    api_script.set_onerror(None);

    // -- append --
    container.append_child(&api_script).unwrap();
    container.append_child(&video_container).unwrap();

    Youtube {
        video_container,
        api_script,
        on_api_loaded,
        on_api_error,
        on_ready,
        player: None,
        on_player_ready: None,
        on_player_state_change: None,
    }
}

#[wasm_bindgen]
extern "C" {
    type YT;

    #[wasm_bindgen(static_method_of = YT)]
    pub fn ready(ready: &js_sys::Function);
}

#[wasm_bindgen]
extern "C" {
    type Player;

    #[wasm_bindgen(constructor, js_namespace = YT)]
    pub fn new(video_container: &web_sys::HtmlElement, config: js_sys::Object) -> Player;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Player);
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct PlayerVars {
    autoplay: u8,
    cc_load_policy: u8,
    controls: u8,
    disablekb: u8,
    enablejsapi: u8,
    fs: u8,
    iv_load_policy: u8,
    r#loop: u8,
    modestbranding: u8,
    playsinline: u8,
    rel: u8,
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    if let Some(player) = &context.core_model.player.selected {
        route_content(
            &model.video_ref, 
            // @TODO make sure `selected` contains `title`
            player.stream.title.as_ref().unwrap_or(&String::new()), 
            context.fullscreen,
            model.playing,
        )
    } else {
        div!["Loading..."]
    }
}

#[view]
fn route_content(video_ref: &ElRef<HtmlElement>, title: &str, fullscreen: bool, playing: bool) -> Node<Msg> {
    div![
        C!["route-content"],
        s()
            .bottom("0")
            .left("0")
            .overflow(CssOverflow::Hidden)
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
        player_container(video_ref, title, fullscreen, playing),
    ]
}

#[view]
fn player_container(video_ref: &ElRef<HtmlElement>, title: &str, fullscreen: bool, playing: bool) -> Node<Msg> {
    div![
        C!["player-container"],
        s()
            .background_color(hsl(0, 0, 0))
            .height(pc(100))
            .position(CssPosition::Relative)
            .width(pc(100))
            .z_index("0"),
        video_container(video_ref),
        overlay(),
        nav_bar(title, fullscreen),
        control_bar(playing),
    ]
}

#[view]
fn video_container(video_ref: &ElRef<HtmlElement>) -> Node<Msg> {
    div![
        C!["layer", "video-container"],
        s()
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
        video(video_ref),
    ]
}

#[view]
fn video(video_ref: &ElRef<HtmlElement>) -> Node<Msg> {
    div![
        C!["video"],
        el_ref(video_ref),
        s()
            .position(CssPosition::Relative)
            .width(pc(100))
            .height(pc(100)),
    ]
}

#[view]
fn overlay() -> Node<Msg> {
    div![
        C!["layer", "overlay"],
        s()
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
    ]
}

