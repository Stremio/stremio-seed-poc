use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
use enclose::enc;
use crate::{PageId, Context, Actions};
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

    load_player(stream, orders);

    let mut model = model.get_or_insert_with(move || Model {
        base_url,
        container_ref: ElRef::new(),
        core_msg_sub_handle: None,
        yt_on_api_loaded: None,
        yt_on_api_error: None,
        yt_on_ready: None,
    });

    if context.core_model.player.selected.is_some() {
        orders.send_msg(Msg::PlayerSelected);
    } else {
        model.core_msg_sub_handle = Some(orders.subscribe_with_handle(Msg::CoreMsg));
    }
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
    core_msg_sub_handle: Option<SubHandle>,
    yt_on_api_loaded: Option<Closure<dyn Fn()>>,
    yt_on_api_error: Option<Closure<dyn Fn()>>,
    yt_on_ready: Option<Rc<Closure<dyn Fn()>>>,
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
    CoreMsg(Rc<CoreMsg>),
    PlayerSelected,
    YoutubeReady(Rc<HtmlElement>),
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CoreMsg(core_msg) => {
            if let CoreMsg::Internal(_) = core_msg.as_ref() {
                if context.core_model.player.selected.is_some() {
                    orders.after_next_render(|_| Msg::PlayerSelected);
                    model.core_msg_sub_handle = None;
                }
            }
        }
        Msg::PlayerSelected => {
            let stream_source = &context.core_model.player.selected.as_ref().unwrap().stream.source;
            match stream_source {
                StreamSource::YouTube { yt_id } => {
                    init_youtube(model, yt_id, orders)
                }
                stream_source => error!("Unhandled stream source:", stream_source),
            }
        }
        Msg::YoutubeReady(video_container) => {
            log!("YoutubeReady!!!")
        }
    }
}

fn init_youtube(model: &mut Model, yt_id: &str, orders: &mut impl Orders<Msg>) {
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
        sender(Some(Msg::YoutubeReady(video_container.clone())));
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
}

#[wasm_bindgen]
extern "C" {
    type YT;

    #[wasm_bindgen(static_method_of = YT)]
    pub fn ready(ready: &js_sys::Function);
}

// #[view]
// fn youtube_video(yt_id: &str) -> Node<Msg> {
//     let params = UrlSearch::new(array::IntoIter::new([
//         ("autoplay", 1),
//         ("cc_load_policy", 3),
//         ("controls", 0),
//         ("disablekb", 1),
//         ("enablejsapi", 1),
//         ("fs", 0),
//         ("iv_load_policy", 3),
//         ("loop", 0),
//         ("modestbranding", 1),
//         ("rel", 0),
//     ]).map(|(key, value)| (key, array::IntoIter::new([value.to_string()]))));

//     div![
//         // https://github.com/paulirish/lite-youtube-embed
//         custom![
//             Tag::from("lite-youtube"),
//             attrs!{
//                 At::from("videoid") => yt_id,
//                 At::from("params") => params
//             }
//         ]
//     ]
// }

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    if context.core_model.player.selected.is_some() {
        div![
            s()
                .background("black")
                .width(pc(100))
                .height(pc(100))
                .color("white"),
            div![
                el_ref(&model.container_ref),
            ],
        ]
    } else {
        div!["Loading..."]
    }
}

