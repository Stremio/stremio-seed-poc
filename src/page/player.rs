use seed::{prelude::*, *};
use stremio_core::types::resource::Stream;

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
//     View
// ------ ------

pub fn view<Ms: 'static>() -> Node<Ms> {
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
