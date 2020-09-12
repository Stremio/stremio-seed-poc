use seed::{prelude::*, *};

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
