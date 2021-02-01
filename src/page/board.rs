use crate::Urls as RootUrls;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(root_base_url: &Url) -> Node<Ms> {
    div![
        h1![
            style! {
                St::Padding => px(20),
            },
            "Board",
        ],
    ]
}
