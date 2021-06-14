use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::array;
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
        video_container_ref: ElRef::new(),
        core_msg_sub_handle: None,
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
    video_container_ref: ElRef<HtmlElement>,
    core_msg_sub_handle: Option<SubHandle>,
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
    PlayerSelected
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
                    render_youtube_player(&model.video_container_ref, yt_id)
                }
                stream_source => error!("Unhandled stream source:", stream_source),
            }
        }
    }
}

fn render_youtube_player(video_container_ref: &ElRef<HtmlElement>, yt_id: &str) {
    let video_container = video_container_ref.get().expect("video container");
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
                el_ref(&model.video_container_ref),
            ],
        ]
    } else {
        div!["Loading..."]
    }
}

