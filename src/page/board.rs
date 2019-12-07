use crate::{page::{discover, addons}, route::Route};
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    div![
        h1![
            style! {
                St::Padding => px(20),
            },
            "Board",
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => Route::Discover(discover::default_resource_request()).to_href()
            },
            "Go to Discover ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => Route::Player.to_href()
            },
            "Go to Player ▶"
        ],
        a![
            style! {
                St::Padding => px(20),
            },
            attrs! {
                At::Href => Route::Addons(addons::default_resource_request()).to_href()
            },
            "Go to Addons ▶"
        ]
    ]
}
