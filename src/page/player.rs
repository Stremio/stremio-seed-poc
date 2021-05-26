use seed::{prelude::*, *};
use crate::{PageId, Context};
use stremio_core::types::resource::Stream;

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
    log!(stream);

    let mut model = model.get_or_insert_with(move || Model {
        base_url,
    });
    Some(PageId::Player)
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

pub fn view(model: &Model) -> Node<Msg> {
    div![video![
        style! {
            St::MaxWidth => unit!(100, %),
            St::Height => "auto",
        },
        attrs! {
            At::Controls => AtValue::None,
        },
        source![attrs! {
            At::Src => "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            At::Type => "video/mp4",
        }]
    ]]
}
