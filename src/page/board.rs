use crate::{default_resource_request, route::Route};
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    div![
        h1!["Board",],
        a![
            style! {
                St::Display => "block",
                St::Margin => px(20),
                St::Padding => px(20),
                St::Background => "black",
                St::TextAlign => "right",
            },
            attrs! {
                At::Href => Route::Discover(default_resource_request()).to_href()
            },
            "Go to Discover ▶"
        ]
    ]
}
