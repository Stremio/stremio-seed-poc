use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
use enclose::enc;
use serde::Serialize;
use crate::{PageId, Context, Actions, Events};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::types::resource::{Stream, StreamSource};
use stremio_core::models::player::Selected as PlayerSelected;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg, Internal};

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
    });
    model.stream = Some(stream);
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
}

pub struct Youtube {
    video_container: Rc<web_sys::HtmlElement>,
    api_script: web_sys::HtmlScriptElement,
    on_api_loaded: Closure<dyn Fn()>,
    on_api_error: Closure<dyn Fn()>,
    on_ready: Rc<Closure<dyn Fn()>>,
    player: Option<Player>,
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
    DestroyPlayer,
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
            let config = PlayerConfig {
                width: "100%",
                height: "100%",
                video_id: &yt_id,
                player_vars: PlayerVars {
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
                },
            };
            let config = serde_wasm_bindgen::to_value(&config).unwrap();
            log!("Youtube config:", config);
            if let Some(youtube) = model.youtube.as_mut() {
                youtube.player = Some(Player::new(&video_container, &config));
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
    pub fn new(video_container: &web_sys::HtmlElement, config: &JsValue) -> Player;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Player);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerConfig<'a> {
    width: &'static str,
    height: &'static str,
    video_id: &'a str,
    player_vars: PlayerVars,
}

#[derive(Serialize)]
struct PlayerVars {
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
    if context.core_model.player.selected.is_some() {
        route_content(model)
    } else {
        div!["Loading..."]
    }
}

#[view]
fn route_content(model: &Model) -> Node<Msg> {
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
        player_container(model),
    ]
}

#[view]
fn player_container(model: &Model) -> Node<Msg> {
    div![
        C!["player-container"],
        s()
            .background_color(hsl(0, 0, 0))
            .height(pc(100))
            .position(CssPosition::Relative)
            .width(pc(100))
            .z_index("0"),
        video_container(model),
        nav_bar(),
        control_bar(),
    ]
}

#[view]
fn video_container(model: &Model) -> Node<Msg> {
    div![
        C!["video-container"],
        s()
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0"),
        video(model),
    ]
}

#[view]
fn video(model: &Model) -> Node<Msg> {
    div![
        C!["video"],
        el_ref(&model.video_ref),
        s()
            .position(CssPosition::Relative)
            .width(pc(100))
            .height(pc(100)),
    ]
}

#[view]
fn nav_bar() -> Node<Msg> {
    nav![
        C!["nav-bar-layer", "horizontal-nav-bar-container"],
        s()
            .background_color("transparent")
            .bottom("initial")
            .overflow(CssOverflow::Visible)
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0")
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .padding_right(rem(1)),
    ]
}

#[view]
fn control_bar() -> Node<Msg> {
    div![
        C!["control-bar-layer", "control-bar-container"],
        s()
            .overflow(CssOverflow::Visible)
            .top("initial")
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .z_index("0")
            .padding("0 1.5rem"),
    ]
}

