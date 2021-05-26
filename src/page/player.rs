use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::rc::Rc;
use std::array;
use crate::{PageId, Context, Actions};
use stremio_core::types::resource::{Stream, StreamSource};
use stremio_core::models::player::Selected as PlayerSelected;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg};

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
    });
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

}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {

    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    match context.core_model.player.selected.as_ref().map(|selected| &selected.stream.source) {
        Some(stream_source) => div![
            s()
                .background("black")
                .width(pc(100))
                .height(pc(100))
                .color("white"),
            video_container(stream_source),
        ],
        None => div!["Loading..."],
    }
}

#[view]
fn video_container(stream_source: &StreamSource) -> Node<Msg> {
    match stream_source {
        StreamSource::YouTube { yt_id } => youtube_video(yt_id),
        _ => div!["Stream not supported"],
    }
}

#[view]
fn youtube_video(yt_id: &str) -> Node<Msg> {
    let params = UrlSearch::new(array::IntoIter::new([
        ("autoplay", 1),
        ("cc_load_policy", 3),
        ("controls", 0),
        ("disablekb", 1),
        ("enablejsapi", 1),
        ("fs", 0),
        ("iv_load_policy", 3),
        ("loop", 0),
        ("modestbranding", 1),
        ("rel", 0),
    ]).map(|(key, value)| (key, array::IntoIter::new([value.to_string()]))));

    div![
        // https://github.com/paulirish/lite-youtube-embed
        custom![
            Tag::from("lite-youtube"),
            attrs!{
                At::from("videoid") => yt_id,
                At::from("params") => params
            }
        ]
    ]
}

