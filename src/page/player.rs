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
        container_ref: ElRef::new(),
        yt_video_container: None,
        yt_api_script: None,
        yt_on_api_loaded: None,
        yt_on_api_error: None,
        yt_on_ready: None,
        yt_player: None,
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
    container_ref: ElRef<HtmlElement>,
    yt_video_container: Option<Rc<web_sys::HtmlElement>>,
    yt_api_script: Option<web_sys::HtmlScriptElement>,
    yt_on_api_loaded: Option<Closure<dyn Fn()>>,
    yt_on_api_error: Option<Closure<dyn Fn()>>,
    yt_on_ready: Option<Rc<Closure<dyn Fn()>>>,
    yt_player: Option<Player>,
    stream: Option<Stream>,
    page_change_sub_handle: SubHandle,
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
            match model.stream.as_ref().unwrap().source.clone() {
                StreamSource::YouTube { yt_id } => {
                    init_youtube(model, yt_id, orders)
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
            model.yt_player = Some(Player::new(&video_container, &config));
        }
        Msg::DestroyPlayer => {
            if let Some(yt_player) = model.yt_player.take() {
                yt_player.destroy();
            }
            if let Some(yt_video_container) = model.yt_video_container.take() {
                yt_video_container.remove();
            }
            if let Some(yt_api_script) = model.yt_api_script.take() {
                yt_api_script.remove();
            }
        }
    }
}

fn init_youtube(model: &mut Model, yt_id: String, orders: &mut impl Orders<Msg>) {
    let container = model.container_ref.get().expect("video container");

    let video_container = document().create_element("div").unwrap().unchecked_into::<web_sys::HtmlElement>();
    let video_container_style = video_container.style();
    video_container_style.set_property("width", "100%").unwrap();
    video_container_style.set_property("height", "100%").unwrap();
    video_container_style.set_property("backgroundColor", "black").unwrap();
    let video_container = Rc::new(video_container);

    let api_script = document().create_element("script").unwrap().unchecked_into::<web_sys::HtmlScriptElement>();
    api_script.set_type("text/javascript");
    api_script.set_src("https://www.youtube.com/iframe_api");

    let sender = orders.msg_sender();
    let on_ready = enc!((video_container) move || {
        sender(Some(Msg::YoutubeReady(video_container.clone(), yt_id.clone())));
    });
    let on_ready = Rc::new(Closure::wrap(Box::new(on_ready) as Box<dyn Fn()>));

    let on_api_loaded = enc!((on_ready) move || {
        YT::ready(on_ready.as_ref().as_ref().unchecked_ref());
    });
    let on_api_loaded = Closure::wrap(Box::new(on_api_loaded) as Box<dyn Fn()>);
    api_script.set_onload(Some(on_api_loaded.as_ref().unchecked_ref()));
    model.yt_on_api_loaded = Some(on_api_loaded);
    model.yt_on_ready = Some(on_ready);

    let on_api_error = || {
        error!("Youtube error");
    };
    let on_api_error = Closure::wrap(Box::new(on_api_error) as Box<dyn Fn()>);
    api_script.set_onerror(None);
    model.yt_on_api_error = Some(on_api_error);

    container.append_child(&api_script).unwrap();
    container.append_child(&video_container).unwrap();

    model.yt_video_container = Some(video_container);
    model.yt_api_script = Some(api_script);
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
        div![
            el_ref(&model.container_ref),
            s()
                .background("black")
                .width(pc(100))
                .height(pc(100))
                .color("white"),
        ]
    } else {
        div!["Loading..."]
    }
}

